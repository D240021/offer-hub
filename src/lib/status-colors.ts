/**
 * Single source of truth for status / severity colors.
 *
 * Every small primitive that paints a state — badges, HTTP method pills, issue
 * difficulty chips, roadmap phase status — resolves its color from here instead
 * of hardcoding hex values, so light and dark mode can never drift apart.
 *
 * Colors are plain CSS color strings rather than Tailwind classes on purpose:
 * the palette lives in CSS custom properties, and Tailwind cannot apply an
 * opacity modifier (`/12`) to a `var()` value, so `bg-theme-success/10` and
 * friends are silently dropped from the bundle. `color-mix()` gives us the same
 * tints while keeping a single theme-aware source.
 */

export type StatusTone = "neutral" | "primary" | "success" | "warning" | "danger";

export interface StatusToneColors {
  /** Text, icons and dots. */
  solid: string;
  /** Soft surface behind a badge or pill. */
  tint: string;
  /** Hairline border that pairs with `tint`. */
  border: string;
}

const fromVariable = (variable: string): StatusToneColors => ({
  solid: `var(${variable})`,
  tint: `color-mix(in srgb, var(${variable}) 12%, transparent)`,
  border: `color-mix(in srgb, var(${variable}) 30%, transparent)`,
});

export const statusColors: Record<StatusTone, StatusToneColors> = {
  neutral: fromVariable("--color-text-muted"),
  primary: fromVariable("--color-primary"),
  success: fromVariable("--color-success"),
  warning: fromVariable("--color-warning"),
  danger: fromVariable("--color-danger"),
};
