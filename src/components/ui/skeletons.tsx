import type { ReactNode } from "react";
import { cn } from "@/lib/cn";
import { Footer } from "@/components/layout/Footer";
import { Navbar } from "@/components/layout/Navbar";
import { LoadingBar } from "@/components/ui/LoadingBar";
import { Skeleton } from "@/components/ui/Skeleton";

/**
 * Building blocks shared by the route-level `loading.tsx` files. Every page
 * skeleton is composed from these so the shimmer, the neumorphic surface and
 * the dark-mode palette stay in one place (see `Skeleton`).
 */

interface PageSkeletonProps {
  className?: string;
  mainClassName?: string;
  children: ReactNode;
}

/** Progress bar + navbar + footer chrome that every page skeleton repeats. */
export function PageSkeleton({ className, mainClassName, children }: PageSkeletonProps) {
  return (
    <div className={className}>
      <LoadingBar />
      <Navbar />
      <main className={mainClassName}>{children}</main>
      <Footer />
    </div>
  );
}

interface SkeletonLinesProps {
  /** One Tailwind width class per line, e.g. `["w-full", "w-5/6"]`. */
  widths: string[];
  className?: string;
}

/**
 * A run of text placeholders. Renders the lines as a fragment so they inherit
 * the `space-y-*` rhythm of whatever container they are dropped into.
 */
export function SkeletonLines({ widths, className }: SkeletonLinesProps) {
  return (
    <>
      {widths.map((width, index) => (
        <Skeleton key={index} className={cn("h-4", className, width)} />
      ))}
    </>
  );
}

interface SkeletonPillRowProps {
  /** Pixel width of each pill, mirroring the real filter labels. */
  widths: number[];
  className?: string;
}

/** Row of filter/tab pills used by the blueprint and use-cases pages. */
export function SkeletonPillRow({ widths, className }: SkeletonPillRowProps) {
  return (
    <div className={cn("flex flex-wrap justify-center gap-3", className)}>
      {widths.map((width, index) => (
        <Skeleton key={index} className="h-10 rounded-2xl" style={{ width }} />
      ))}
    </div>
  );
}

/** Title plus body copy block used by the legal pages (terms, privacy). */
export function LegalSectionSkeleton() {
  return (
    <div className="p-8 md:p-12 rounded-[2.5rem] space-y-4">
      <Skeleton className="h-8 w-48" />
      <SkeletonLines widths={["w-full", "w-5/6", "w-4/5", "w-full", "w-3/4"]} />
    </div>
  );
}

interface PageHeroSkeletonProps {
  /** Width of the eyebrow pill, the only difference between hero layouts. */
  badgeClassName: string;
}

/** Oversized centered hero shared by the architecture and blueprint pages. */
export function PageHeroSkeleton({ badgeClassName }: PageHeroSkeletonProps) {
  return (
    <section className="relative pt-44 md:pt-48 pb-20 bg-transparent">
      <div className="max-w-7xl mx-auto px-6 lg:px-8 text-center flex flex-col items-center">
        <Skeleton className="h-[clamp(3rem,10vw,7.75rem)] w-2/3 rounded-2xl mb-6" />
        <Skeleton className={cn("h-8 rounded-full mb-8", badgeClassName)} />
        <Skeleton className="h-16 w-3/4 rounded-xl mb-3" />
        <Skeleton className="h-16 w-2/3 rounded-xl mb-8" />
        <Skeleton className="h-5 w-2/3 mb-2" />
        <Skeleton className="h-5 w-1/2 mb-12" />
        <Skeleton className="w-full max-w-3xl rounded-[2rem] h-64" />
      </div>
    </section>
  );
}
