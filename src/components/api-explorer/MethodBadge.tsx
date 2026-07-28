import { cn } from "@/lib/cn";
import { Badge } from "@/components/ui/Badge";
import type { StatusTone } from "@/lib/status-colors";
import type { HttpMethod } from "@/data/api-schema";

const METHOD_TONES: Record<HttpMethod, StatusTone> = {
  GET: "success",
  POST: "primary",
  PUT: "warning",
  DELETE: "danger",
};

interface MethodBadgeProps {
  method: HttpMethod;
  className?: string;
}

export function MethodBadge({ method, className }: MethodBadgeProps) {
  return (
    <Badge variant={METHOD_TONES[method]} elevated={false} className={cn("font-mono", className)}>
      {method}
    </Badge>
  );
}
