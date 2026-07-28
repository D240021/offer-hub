"use client";

import { useEffect, useState } from "react";
import { cn } from "@/lib/cn";
import { useScrollProgress } from "@/hooks/useScrollProgress";
import { TOP_PROGRESS_GRADIENT, TopProgressBar } from "@/components/ui/TopProgressBar";

/**
 * Reading progress for a single page: plays a short intro fill on mount, then
 * mirrors how far down the document the visitor has scrolled. Opt in per page.
 * For route-change feedback see `NavigationProgress`, mounted in the layout.
 */
export function LoadingBar() {
    const [progress, setProgress] = useState(0);
    const [isLoading, setIsLoading] = useState(true);
    const scrollY = useScrollProgress(!isLoading);

    useEffect(() => {
        setProgress(0.3);
        const timer = setTimeout(() => {
            setProgress(1);
            setTimeout(() => setIsLoading(false), 500);
        }, 800);
        return () => clearTimeout(timer);
    }, []);

    useEffect(() => {
        if (isLoading) return;
        const docHeight = document.documentElement.scrollHeight - window.innerHeight;
        setProgress(docHeight > 0 ? scrollY / docHeight : 0);
    }, [isLoading, scrollY]);

    return (
        <TopProgressBar
            className={cn(TOP_PROGRESS_GRADIENT, "origin-left")}
            style={{ transform: `scaleX(${progress})`, transition: isLoading ? "transform 0.4s ease" : "none" }}
        />
    );
}
