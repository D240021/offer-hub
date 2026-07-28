import { Skeleton } from "@/components/ui/Skeleton";
import { LegalSectionSkeleton, PageSkeleton, SkeletonLines } from "@/components/ui/skeletons";

export default function TermsLoading() {
  return (
    <PageSkeleton
      className="min-h-screen flex flex-col bg-bg-base"
      mainClassName="flex-grow pt-32 pb-24 px-6 lg:px-8"
    >
      <div className="max-w-4xl mx-auto">
        <header className="text-center mb-20 space-y-4">
          <Skeleton className="h-3 w-24 mx-auto" />
          <Skeleton className="h-12 w-2/3 mx-auto" />
          <Skeleton className="h-5 w-full max-w-2xl mx-auto" />
          <Skeleton className="h-8 w-48 rounded-full mx-auto mt-4" />
        </header>

        <div className="space-y-12">
          {[1, 2, 3, 4, 5].map((i) => (
            <LegalSectionSkeleton key={i} />
          ))}

          <div className="p-8 md:p-10 rounded-[2.5rem] space-y-3">
            <SkeletonLines className="h-3" widths={["w-full", "w-3/4", "w-full"]} />
          </div>
        </div>
      </div>
    </PageSkeleton>
  );
}
