import type { CSSProperties, ReactNode } from "react";
import { cn } from "@/lib/cn";
import { statusColors, type StatusTone } from "@/lib/status-colors";

export type BadgeShape = "pill" | "square";

export interface BadgeProps {
  variant?: StatusTone;
  shape?: BadgeShape;
  /** Hairline border in the variant color. */
  bordered?: boolean;
  /** Neumorphic lift. Turn off for badges nested inside an already raised card. */
  elevated?: boolean;
  className?: string;
  children: ReactNode;
}

const SHAPE_STYLES: Record<BadgeShape, string> = {
  pill: "rounded-full px-2.5 py-0.5",
  square: "rounded-md px-2 py-0.5",
};

export function Badge({
  variant = "neutral",
  shape = "pill",
  bordered = false,
  elevated = true,
  className,
  children,
}: BadgeProps) {
  const colors = statusColors[variant];

  const style: CSSProperties = { color: colors.solid, background: colors.tint };
  if (bordered) {
    style.border = `1px solid ${colors.border}`;
  }

  return (
    <span
      className={cn(
        "inline-flex items-center text-xs font-semibold",
        SHAPE_STYLES[shape],
        elevated && "shadow-neu-raised-sm",
        className,
      )}
      style={style}
    >
      {children}
    </span>
  );
}
