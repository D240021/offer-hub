import { CheckCircle2, Lock, Loader2 } from "lucide-react";
import type { PhaseDomain, PhaseStatus, StatusConfig } from "./types";

export const NEU_ELEVATED =
  "shadow-[6px_6px_14px_var(--shadow-dark),-6px_-6px_14px_var(--shadow-light)]";

export const NEU_SUNKEN =
  "shadow-[inset_4px_4px_10px_var(--shadow-dark),inset_-4px_-4px_10px_var(--shadow-light)]";

export const DOMAINS: { key: "all" | PhaseDomain; label: string }[] = [
  { key: "all", label: "All Phases" },
  { key: "core", label: "Core (0-7)" },
  { key: "sdk", label: "SDK (Phase 8)" },
  { key: "qa", label: "QA (Phase 9)" },
  { key: "crypto", label: "Crypto-Native (Phase 10)" },
];

const STATUS_CONFIG: Record<PhaseStatus, StatusConfig> = {
  completed: { icon: CheckCircle2, label: "Completed", tone: "success" },
  "in-progress": { icon: Loader2, label: "In Progress", tone: "primary" },
  planned: { icon: Lock, label: "Planned", tone: "neutral" },
};

export function statusConfig(status: PhaseStatus): StatusConfig {
  return STATUS_CONFIG[status];
}
