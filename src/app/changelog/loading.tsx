import { Skeleton } from "@/components/ui/Skeleton";
import { PageSkeleton, SkeletonLines } from "@/components/ui/skeletons";

export default function ChangelogLoading() {
  return (
    <PageSkeleton
      className="min-h-screen flex flex-col bg-transparent"
      mainClassName="flex-grow pt-32 pb-24 px-6 md:px-8"
    >
      <div className="max-w-4xl mx-auto">
        <header className="text-center mb-16 space-y-4">
          <Skeleton className="h-3 w-24 mx-auto" />
          <Skeleton className="h-12 md:h-14 w-2/3 mx-auto" />
          <Skeleton className="h-5 w-1/2 mx-auto" />
        </header>

        <div className="space-y-8">
          {[1, 2, 3, 4].map((i) => (
            <div key={i} className="rounded-[2.5rem] p-8 space-y-3">
              <Skeleton className="h-6 w-1/3" />
              <SkeletonLines widths={["w-full", "w-5/6", "w-4/6"]} />
            </div>
          ))}
        </div>
      </div>
    </PageSkeleton>
  );
}
