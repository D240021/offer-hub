import { Skeleton } from "@/components/ui/Skeleton";
import { PageHeroSkeleton, PageSkeleton } from "@/components/ui/skeletons";

export default function ArchitectureLoading() {
  return (
    <PageSkeleton className="min-h-screen flex flex-col bg-transparent" mainClassName="flex-grow">
      <PageHeroSkeleton badgeClassName="w-48" />

      <section className="px-6 py-24 bg-transparent">
        <div className="mx-auto max-w-7xl grid md:grid-cols-2 gap-8">
          {[1, 2, 3, 4].map((i) => (
            <Skeleton key={i} className="rounded-[2.5rem] h-96" />
          ))}
        </div>
      </section>
    </PageSkeleton>
  );
}
