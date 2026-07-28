import type { CSSProperties, ReactNode } from "react";
import { cn } from "@/lib/cn";

/**
 * The teal rail shared by the two top-of-viewport progress indicators:
 *
 * - `NavigationProgress` — route-change progress, mounted once in the root
 *   layout. It fills left to right while the next route loads.
 * - `LoadingBar` — scroll progress, opted into per page. It plays a short
 *   intro fill and then tracks how far down the document you are.
 *
 * They intentionally stay separate components because they answer different
 * questions ("is a page loading?" vs "how far have I read?"), but they must
 * look identical, so the rail geometry and gradient live here.
 */

export const TOP_PROGRESS_GRADIENT = "bg-gradient-to-r from-[#149A9B] to-[#22e0e2]";

interface TopProgressBarProps {
  className?: string;
  style?: CSSProperties;
  children?: ReactNode;
}

export function TopProgressBar({ className, style, children }: TopProgressBarProps) {
  return (
    <div
      className={cn("fixed top-0 left-0 right-0 h-[3px] z-[9999]", className)}
      style={style}
    >
      {children}
    </div>
  );
}
