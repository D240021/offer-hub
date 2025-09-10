use soroban_sdk::{Address, Env, IntoVal, String, Symbol, Vec};

use crate::{
    error::handle_error,
    storage::{ESCROW_DATA, INITIALIZED, add_call_log, CallLog, CONTRACT_CONFIG, 
              DEFAULT_MIN_ESCROW_AMOUNT, DEFAULT_MAX_ESCROW_AMOUNT, DEFAULT_TIMEOUT_DAYS,
              DEFAULT_MAX_MILESTONES, DEFAULT_FEE_PERCENTAGE, DEFAULT_RATE_LIMIT_CALLS,
              DEFAULT_RATE_LIMIT_WINDOW_HOURS},
    types::{DisputeResult, Error, EscrowData, EscrowStatus, Milestone, MilestoneHistory, ContractConfig},
    validation::{validate_init_contract, validate_init_contract_full, validate_add_milestone, validate_milestone_id, validate_address},
};
use crate::storage::{check_rate_limit, set_rate_limit_bypass_flag, reset_rate_limit as rl_reset};

const TOKEN_TRANSFER: &str = "transfer";
const TOKEN_BALANCE: &str = "balance";

pub fn initialize_contract(env: &Env, admin: Address) {
    if env.storage().instance().has(&CONTRACT_CONFIG) {
        handle_error(env, Error::AlreadyInitialized);
    }
    
    admin.require_auth();
    
    let contract_config = ContractConfig {
        min_escrow_amount: DEFAULT_MIN_ESCROW_AMOUNT,
        max_escrow_amount: DEFAULT_MAX_ESCROW_AMOUNT,
        default_timeout_days: DEFAULT_TIMEOUT_DAYS,
        max_milestones: DEFAULT_MAX_MILESTONES,
        fee_percentage: DEFAULT_FEE_PERCENTAGE,
        rate_limit_calls: DEFAULT_RATE_LIMIT_CALLS,
        rate_limit_window_hours: DEFAULT_RATE_LIMIT_WINDOW_HOURS,
    };
    
    env.storage().instance().set(&CONTRACT_CONFIG, &contract_config);
    
    env.events().publish(
        (Symbol::new(env, "contract_initialized"), admin),
        env.ledger().timestamp(),
    );
}

// Helper function to log function calls
fn log_function_call(
    env: &Env,
    function_name: &str,
    caller: &Address,
    success: bool,
) {
    let log = CallLog {
        function_name: String::from_str(env, function_name),
        caller: caller.clone(),
        timestamp: env.ledger().timestamp(),
        success,
        gas_used: 0, // Soroban doesn't provide gas_used method
        transaction_hash: String::from_str(env, "unknown"), // Soroban doesn't provide transaction_hash method
    };
    
    add_call_log(env, &log);
}

pub fn init_contract_full(
    env: &Env,
    client: Address,
    freelancer: Address,
    arbitrator: Address,
    token: Address,
    amount: i128,
    timeout_secs: u64,
) {
    let caller = client.clone();
    
    // Log function call start
    log_function_call(env, "init_contract_full", &caller, true);
    
    if env.storage().instance().has(&INITIALIZED) {
        handle_error(env, Error::AlreadyInitialized);
    }
    
    // Input validation
    if let Err(e) = validate_init_contract_full(env, &client, &freelancer, &arbitrator, &token, amount, timeout_secs) {
        handle_error(env, e);
    }
    let escrow_data = EscrowData {
        client: client.clone(),
        freelancer,
        arbitrator: Some(arbitrator),
        token: Some(token),
        amount,
        status: EscrowStatus::Initialized,
        dispute_result: DisputeResult::None as u32,
        created_at: env.ledger().timestamp(),
        funded_at: None,
        released_at: None,
        disputed_at: None,
        resolved_at: None,
        timeout_secs: Some(timeout_secs),
        milestones: Vec::new(env),
        milestone_history: Vec::new(env),
        released_amount: 0,
        fee_manager: client,
        fee_collected: 0,
        net_amount: amount,
    };
    env.storage().instance().set(&ESCROW_DATA, &escrow_data);
    env.storage().instance().set(&INITIALIZED, &true);
}

pub fn init_contract(
    env: &Env,
    client: Address,
    freelancer: Address,
    amount: i128,
    fee_manager: Address,
) {
    let caller = client.clone();
    
    // Log function call start
    log_function_call(env, "init_contract", &caller, true);
    
    if env.storage().instance().has(&INITIALIZED) {
        handle_error(env, Error::AlreadyInitialized);
    }

    // Input validation
    if let Err(e) = validate_init_contract(env, &client, &freelancer, amount, &fee_manager) {
        handle_error(env, e);
    }

    let escrow_data = EscrowData {
        client,
        freelancer,
        arbitrator: None,
        token: None,
        amount,
        status: EscrowStatus::Initialized,
        dispute_result: DisputeResult::None as u32,
        created_at: env.ledger().timestamp(),
        funded_at: None,
        released_at: None,
        disputed_at: None,
        resolved_at: None,
        timeout_secs: None,
        milestones: Vec::new(env),
        milestone_history: Vec::new(env),
        released_amount: 0,
        fee_manager: fee_manager,
        fee_collected: 0,
        net_amount: amount,
    };

    env.storage().instance().set(&ESCROW_DATA, &escrow_data);
    env.storage().instance().set(&INITIALIZED, &true);
}

pub fn deposit_funds(env: &Env, client: Address) {
    let caller = client.clone();
    
    // Log function call start
    log_function_call(env, "deposit_funds", &caller, true);
    
    client.require_auth();

    if !env.storage().instance().has(&INITIALIZED) {
        handle_error(env, Error::NotInitialized);
    }

    let mut escrow_data: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();

    if escrow_data.client != client {
        handle_error(env, Error::Unauthorized);
    }

    if escrow_data.status != EscrowStatus::Initialized {
        handle_error(env, Error::InvalidStatus);
    }

    if let (Some(token), amount) = (escrow_data.token.clone(), escrow_data.amount) {
        let balance: i128 = env.invoke_contract::<i128>(
            &token,
            &Symbol::new(env, TOKEN_BALANCE),
            (client.clone(),).into_val(env),
        );
        if balance < amount {
            handle_error(env, Error::InsufficientFunds);
        }

        env.invoke_contract::<()>(
            &token,
            &Symbol::new(env, TOKEN_TRANSFER),
            (client.clone(), env.current_contract_address(), amount).into_val(env),
        );
    }

    escrow_data.status = EscrowStatus::Funded;
    escrow_data.funded_at = Some(env.ledger().timestamp());

    env.storage().instance().set(&ESCROW_DATA, &escrow_data);

    env.events().publish(
        (Symbol::new(env, "deposited_fund"), client.clone()),
        (escrow_data.amount, env.ledger().timestamp()),
    );
}

pub fn release_funds(env: &Env, freelancer: Address) {
    let caller = freelancer.clone();
    
    // Log function call start
    log_function_call(env, "release_funds", &caller, true);
    
    freelancer.require_auth();

    if !env.storage().instance().has(&INITIALIZED) {
        handle_error(env, Error::NotInitialized);
    }

    let mut escrow_data: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();

    if escrow_data.freelancer != freelancer {
        handle_error(env, Error::Unauthorized);
    }

    if escrow_data.status != EscrowStatus::Funded {
        handle_error(env, Error::InvalidStatus);
    }

    if let (Some(token), amount) = (escrow_data.token.clone(), escrow_data.amount) {
        env.invoke_contract::<()>(
            &token,
            &Symbol::new(env, TOKEN_TRANSFER),
            (env.current_contract_address(), freelancer.clone(), amount).into_val(env),
        );
    }

    let fee_percentage = 250;
    let fee_amount = (escrow_data.amount * fee_percentage) / 10000;
    let net_amount = escrow_data.amount - fee_amount;

    escrow_data.status = EscrowStatus::Released;
    escrow_data.released_at = Some(env.ledger().timestamp());
    escrow_data.fee_collected = fee_amount;
    escrow_data.net_amount = net_amount;

    env.storage().instance().set(&ESCROW_DATA, &escrow_data);

    env.events().publish(
        (Symbol::new(env, "released_fund"), freelancer.clone()),
        (
            escrow_data.amount,
            escrow_data.net_amount,
            escrow_data.fee_collected,
            escrow_data.client,
            env.ledger().timestamp(),
        ),
    );
}

pub fn dispute(env: &Env, caller: Address) {
    let caller_addr = caller.clone();
    
    // Log function call start
    log_function_call(env, "dispute", &caller_addr, true);
    
    caller.require_auth();

    if !env.storage().instance().has(&INITIALIZED) {
        handle_error(env, Error::NotInitialized);
    }

    let mut escrow_data: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();

    if escrow_data.client != caller && escrow_data.freelancer != caller {
        handle_error(env, Error::Unauthorized);
    }

    if escrow_data.status != EscrowStatus::Funded {
        handle_error(env, Error::InvalidStatus);
    }

    escrow_data.status = EscrowStatus::Disputed;
    escrow_data.disputed_at = Some(env.ledger().timestamp());

    env.storage().instance().set(&ESCROW_DATA, &escrow_data);

    env.events().publish(
        (Symbol::new(env, "escrow_disputed"), caller.clone()),
        env.ledger().timestamp(),
    );
}

pub fn resolve_dispute(env: &Env, caller: Address, result: Symbol) {
    let caller_addr = caller.clone();
    
    // Log function call start
    log_function_call(env, "resolve_dispute", &caller_addr, true);
    
    caller.require_auth();

    if !env.storage().instance().has(&INITIALIZED) {
        handle_error(env, Error::NotInitialized);
    }

    let mut escrow_data: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();

    if escrow_data.status != EscrowStatus::Disputed {
        handle_error(env, Error::DisputeNotOpen);
    }

    if escrow_data.arbitrator != Some(caller.clone()) {
        handle_error(env, Error::Unauthorized);
    }

    // CORREGIDO: Aceptar más variaciones de símbolos
    let dispute_result = if result == Symbol::new(env, "client_wins") || result == Symbol::new(env, "client") {
        DisputeResult::ClientWins
    } else if result == Symbol::new(env, "freelancer_wins") || result == Symbol::new(env, "freelancer") {
        DisputeResult::FreelancerWins
    } else if result == Symbol::new(env, "split") {
        DisputeResult::Split
    } else {
        handle_error(env, Error::InvalidDisputeResult);
    };

    if let (Some(token), amount) = (escrow_data.token.clone(), escrow_data.amount) {
        let contract_addr = env.current_contract_address();
        match dispute_result {
            DisputeResult::ClientWins => {
                env.invoke_contract::<()>(
                    &token,
                    &Symbol::new(env, TOKEN_TRANSFER),
                    (contract_addr.clone(), escrow_data.client.clone(), amount).into_val(env),
                );
            }
            DisputeResult::FreelancerWins => {
                env.invoke_contract::<()>(
                    &token,
                    &Symbol::new(env, TOKEN_TRANSFER),
                    (
                        contract_addr.clone(),
                        escrow_data.freelancer.clone(),
                        amount,
                    )
                        .into_val(env),
                );
            }
            DisputeResult::Split => {
                let half = amount / 2;
                env.invoke_contract::<()>(
                    &token,
                    &Symbol::new(env, TOKEN_TRANSFER),
                    (contract_addr.clone(), escrow_data.client.clone(), half).into_val(env),
                );
                env.invoke_contract::<()>(
                    &token,
                    &Symbol::new(env, TOKEN_TRANSFER),
                    (
                        contract_addr.clone(),
                        escrow_data.freelancer.clone(),
                        amount - half,
                    )
                        .into_val(env),
                );
            }
            DisputeResult::None => {
                handle_error(env, Error::InvalidDisputeResult);
            }
        }
    }

    escrow_data.status = EscrowStatus::Released; // CORREGIDO: cambiar a Released en lugar de Resolved
    escrow_data.dispute_result = dispute_result as u32;
    escrow_data.resolved_at = Some(env.ledger().timestamp());

    env.storage().instance().set(&ESCROW_DATA, &escrow_data);

    env.events().publish(
        (Symbol::new(env, "escrow_resolved"), result.clone()),
        env.ledger().timestamp(),
    );
}

pub fn add_milestone(env: &Env, client: Address, desc: String, amount: i128) -> u32 {
    let caller = client.clone();
    
    // Log function call start
    log_function_call(env, "add_milestone", &caller, true);
    
    client.require_auth();
    
    // Input validation
    if let Err(e) = validate_add_milestone(env, &client, &desc, amount) {
        handle_error(env, e);
    }
    
    let mut escrow: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();

    // Rate limit: max 10 milestones per hour per client for this escrow
    let limit_type = String::from_str(env, "add_milestone");
    if let Err(e) = check_rate_limit(env, &client, &limit_type, 10, 3600) {
        handle_error(env, e);
    }

    if escrow.client != client {
        handle_error(env, Error::Unauthorized);
    }

    let m_id = (escrow.milestones.len() as u32) + 1;
    let ts = env.ledger().timestamp();

    let milestone = Milestone {
        id: m_id,
        description: desc.clone(),
        amount,
        approved: false,
        released: false,
        created_at: ts,
        approved_at: None,
        released_at: None,
    };

    escrow.milestones.push_back(milestone.clone());
    escrow.milestone_history.push_back(MilestoneHistory {
        milestone: milestone,
        action: String::from_str(env, "added"),
        timestamp: ts,
    });

    env.storage().instance().set(&ESCROW_DATA, &escrow);

    env.events().publish(
        (Symbol::new(env, "escrow_milestone_added"), client.clone()),
        (m_id, desc.clone(), amount, env.ledger().timestamp()),
    );
    m_id
}

// Admin helpers for rate limiting: admin is the escrow client
pub fn set_rate_limit_bypass(env: &Env, caller: Address, user: Address, bypass: bool) {
    caller.require_auth();
    // Only escrow client can toggle bypass
    let escrow: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();
    if escrow.client != caller { handle_error(env, Error::Unauthorized); }
    set_rate_limit_bypass_flag(env, &user, bypass);
}

pub fn reset_rate_limit(env: &Env, caller: Address, user: Address, limit_type: String) {
    caller.require_auth();
    let escrow: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();
    if escrow.client != caller { handle_error(env, Error::Unauthorized); }
    rl_reset(env, &user, &limit_type);
}

// CORREGIDO: usar índice correcto (milestone_id - 1)
pub fn approve_milestone(env: &Env, client: Address, milestone_id: u32) {
    let caller = client.clone();
    
    // Log function call start
    log_function_call(env, "approve_milestone", &caller, true);
    
    client.require_auth();
    
    // Input validation
    if let Err(_) = validate_address(&client) {
        handle_error(env, Error::Unauthorized);
    }
    if let Err(e) = validate_milestone_id(milestone_id) {
        handle_error(env, e);
    }
    
    let mut escrow: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();
    let ts = env.ledger().timestamp();

    if escrow.client != client {
        handle_error(env, Error::Unauthorized);
    }

    // CORREGIDO: convertir milestone_id (base-1) a índice (base-0)
    let index = milestone_id - 1;

    if index >= escrow.milestones.len() {
        handle_error(env, Error::MilestoneNotFound);
    }

    let mut milestone = escrow.milestones.get(index).unwrap();

    if milestone.approved {
        handle_error(env, Error::InvalidStatus);
    }

    milestone.approved = true;
    milestone.approved_at = Some(ts);
    
    // CORREGIDO: actualizar el milestone en el vector
    escrow.milestones.set(index, milestone.clone());
    
    escrow.milestone_history.push_back(MilestoneHistory {
        milestone: milestone.clone(),
        action: String::from_str(env, "approved"),
        timestamp: ts,
    });
    env.storage().instance().set(&ESCROW_DATA, &escrow);

    env.events().publish(
        (
            Symbol::new(env, "escrow_milestone_approved"),
            client.clone(),
        ),
        (milestone_id, env.ledger().timestamp()),
    );
}

// CORREGIDO: usar índice correcto (milestone_id - 1)
pub fn release_milestone(env: &Env, freelancer: Address, milestone_id: u32) {
    let caller = freelancer.clone();
    
    // Log function call start
    log_function_call(env, "release_milestone", &caller, true);
    
    freelancer.require_auth();
    
    // Input validation
    if let Err(_) = validate_address(&freelancer) {
        handle_error(env, Error::Unauthorized);
    }
    if let Err(e) = validate_milestone_id(milestone_id) {
        handle_error(env, e);
    }
    
    let mut escrow: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();
    let ts = env.ledger().timestamp();

    if escrow.freelancer != freelancer {
        handle_error(env, Error::Unauthorized);
    }

    // CORREGIDO: convertir milestone_id (base-1) a índice (base-0)
    let index = milestone_id - 1;

    if index >= escrow.milestones.len() {
        handle_error(env, Error::MilestoneNotFound);
    }

    let mut milestone = escrow.milestones.get(index).unwrap();

    if !milestone.approved || milestone.released {
        handle_error(env, Error::InvalidStatus);
    }

    milestone.released = true;
    milestone.released_at = Some(ts);
    escrow.released_amount += milestone.amount;
    
    // CORREGIDO: actualizar el milestone en el vector
    escrow.milestones.set(index, milestone.clone());
    
    escrow.milestone_history.push_back(MilestoneHistory {
        milestone: milestone.clone(),
        action: String::from_str(env, "released"),
        timestamp: ts,
    });
    env.storage().instance().set(&ESCROW_DATA, &escrow);

    env.events().publish(
        (
            Symbol::new(env, "escrow_milestone_released"),
            freelancer.clone(),
        ),
        (milestone_id, milestone.amount, ts),
    );
}

pub fn get_escrow_data(env: &Env) -> EscrowData {
    if !env.storage().instance().has(&INITIALIZED) {
        handle_error(env, Error::NotInitialized);
    }

    env.storage().instance().get(&ESCROW_DATA).unwrap()
}

pub fn auto_release(env: &Env) {
    if !env.storage().instance().has(&INITIALIZED) {
        handle_error(env, Error::NotInitialized);
    }
    let mut escrow_data: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();
    if escrow_data.status != EscrowStatus::Funded {
        handle_error(env, Error::InvalidStatus);
    }
    let funded_at = escrow_data.funded_at.unwrap_or(0);
    let timeout = escrow_data.timeout_secs.unwrap_or(0);
    let now = env.ledger().timestamp();
    if now < funded_at + timeout {
        handle_error(env, Error::InvalidStatus);
    }
    
    if let (Some(token), amount) = (escrow_data.token.clone(), escrow_data.amount) {
        env.invoke_contract::<()>(
            &token,
            &Symbol::new(env, TOKEN_TRANSFER),
            (
                env.current_contract_address(),
                escrow_data.freelancer.clone(),
                amount,
            )
                .into_val(env),
        );
    }
    escrow_data.status = EscrowStatus::Released;
    escrow_data.released_at = Some(now);

    env.storage().instance().set(&ESCROW_DATA, &escrow_data);

    env.events().publish(
        (Symbol::new(env, "auto_released"), escrow_data.freelancer.clone()),
        (escrow_data.amount, now),
    );
}

pub fn get_milestones(env: &Env) -> Vec<Milestone> {
    let escrow: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();
    escrow.milestones.clone()
}

pub fn get_milestone_history(env: &Env) -> Vec<MilestoneHistory> {
    let escrow: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();
    escrow.milestone_history.clone()
}

pub fn set_escrow_data(env: &Env, data: &EscrowData) {
    env.storage().instance().set(&ESCROW_DATA, data);
}

// Logging functions
pub fn get_call_logs(env: &Env) -> Vec<CallLog> {
    crate::storage::get_call_logs(env)
}

pub fn clear_call_logs(env: &Env, caller: Address) {
    caller.require_auth();
    
    // Only escrow client can clear logs
    let escrow: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();
    if escrow.client != caller { 
        handle_error(env, Error::Unauthorized); 
    }
    
    crate::storage::clear_call_logs(env);
}

pub fn set_config(env: &Env, caller: Address, config: ContractConfig) {
    caller.require_auth();
    
    // Only escrow client can set config (admin functionality)
    let escrow: EscrowData = env.storage().instance().get(&ESCROW_DATA).unwrap();
    if escrow.client != caller { 
        handle_error(env, Error::Unauthorized); 
    }
    
    // Validate config parameters
    if let Err(e) = validate_config(&config) {
        handle_error(env, e);
    }
    
    env.storage().instance().set(&CONTRACT_CONFIG, &config);
    
    env.events().publish(
        (Symbol::new(env, "cfg_upd"), caller),
        (config.min_escrow_amount, config.max_escrow_amount, config.default_timeout_days),
    );
}

pub fn get_config(env: &Env) -> ContractConfig {
    if !env.storage().instance().has(&CONTRACT_CONFIG) {
        handle_error(env, Error::NotInitialized);
    }
    env.storage().instance().get(&CONTRACT_CONFIG).unwrap()
}

// Helper function to validate config parameters
fn validate_config(config: &ContractConfig) -> Result<(), Error> {
    // Validate escrow amounts
    if config.min_escrow_amount >= config.max_escrow_amount {
        return Err(Error::InvalidAmount);
    }
    
    if config.min_escrow_amount < 1 {
        return Err(Error::InvalidAmount);
    }
    
    // Validate timeout (1-365 days)
    if config.default_timeout_days < 1 || config.default_timeout_days > 365 {
        return Err(Error::InvalidAmount);
    }
    
    // Validate max milestones (1-100)
    if config.max_milestones < 1 || config.max_milestones > 100 {
        return Err(Error::InvalidAmount);
    }
    
    // Validate fee percentage (0-10%)
    if config.fee_percentage > 1000 {
        return Err(Error::InvalidAmount);
    }
    
    // Validate rate limit parameters
    if config.rate_limit_window_hours < 1 || config.rate_limit_window_hours > 168 {
        return Err(Error::InvalidAmount);
    }
    
    if config.rate_limit_calls < 1 || config.rate_limit_calls > 1000 {
        return Err(Error::InvalidAmount);
    }
    
    Ok(())
}