import type { ElementType } from "react";
import type { StatusTone } from "@/lib/status-colors";

export type PhaseStatus = "completed" | "in-progress" | "planned";
export type PhaseDomain = "core" | "sdk" | "qa" | "crypto";

export interface Deliverable {
  id: string;
  label: string;
  done: boolean;
}

export interface Phase {
  id: number;
  phase: string;
  title: string;
  goal: string;
  status: PhaseStatus;
  completion: number;
  domain: PhaseDomain;
  icon: ElementType;
  dateTag?: string;
  deliverables: Deliverable[];
}

export interface StatusConfig {
  icon: ElementType;
  label: string;
  /** Key into the shared `statusColors` palette. */
  tone: StatusTone;
}
