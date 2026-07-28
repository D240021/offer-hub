import { PageHeroSkeleton, PageSkeleton, SkeletonPillRow } from "@/components/ui/skeletons";

export default function BlueprintLoading() {
  return (
    <PageSkeleton className="min-h-screen flex flex-col bg-transparent" mainClassName="flex-grow">
      <PageHeroSkeleton badgeClassName="w-40" />

      <section className="px-6 py-24 bg-transparent">
        <div className="mx-auto max-w-6xl">
          <SkeletonPillRow widths={[120, 140, 128, 152, 136]} />
        </div>
      </section>
    </PageSkeleton>
  );
}
