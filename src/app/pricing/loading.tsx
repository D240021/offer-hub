import { Skeleton } from "@/components/ui/Skeleton";
import { PageSkeleton, SkeletonLines } from "@/components/ui/skeletons";

function PricingCardSkeleton() {
  return (
    <div className="rounded-[2.5rem] p-10 flex flex-col space-y-5">
      <Skeleton className="w-16 h-16 rounded-2xl" />
      <Skeleton className="h-7 w-36" />
      <Skeleton className="h-3 w-24" />
      <div className="space-y-1.5">
        <SkeletonLines widths={["w-full", "w-5/6", "w-4/6"]} />
      </div>
      <ul className="space-y-3 flex-grow">
        {[1, 2, 3, 4].map((i) => (
          <li key={i} className="flex items-start gap-2">
            <Skeleton className="mt-1.5 h-1.5 w-1.5 rounded-full shrink-0" />
            <Skeleton className="h-4 w-full" />
          </li>
        ))}
      </ul>
      <Skeleton className="h-12 w-full rounded-xl" />
    </div>
  );
}

export default function PricingLoading() {
  return (
    <PageSkeleton
      className="min-h-screen flex flex-col bg-transparent"
      mainClassName="flex-grow pt-28 pb-20"
    >
      <section className="max-w-7xl mx-auto px-6 lg:px-8">
        <div className="text-center max-w-3xl mx-auto flex flex-col items-center space-y-3">
          <Skeleton className="h-8 w-28 rounded-full mb-5" />
          <Skeleton className="h-14 w-3/4 rounded-xl" />
          <Skeleton className="h-14 w-2/3 rounded-xl" />
          <SkeletonLines className="h-5" widths={["w-full", "w-5/6", "w-4/6"]} />
        </div>

        <div className="mt-14 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {[1, 2, 3].map((i) => (
            <PricingCardSkeleton key={i} />
          ))}
        </div>

        <div className="mt-10 flex justify-center">
          <Skeleton className="h-4 w-80" />
        </div>
      </section>
    </PageSkeleton>
  );
}
