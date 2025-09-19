"use client"

import { useAuthGuard } from "@/hooks/use-auth-guard";
import { ReactNode } from "react"
import { LoadingSpinner } from "../LoadingSpinner";
import { AccessDenied } from "./AccessDenied";




interface AuthGuardProps {
    children: ReactNode;
    roles?: string[]
}


export const AuthGuard = ({ children, roles }: AuthGuardProps) => {

    const { isLoading, hasAccess, error } = useAuthGuard(roles)


    if (isLoading) {
        return (
            <div className="w-full h-screen flex items-center flex-col justify-center gap-3 " >

                <LoadingSpinner />
                <span className=" text-teal-600">Verifying access…</span>
            </div>
        )
    }


    if (error) {
        return (
            <div className="w-full h-screen flex items-center justify-center gap-3 ">
                <p className="text-red-600 font-medium">{error}</p>
            </div>
        )
    }

    if (!hasAccess) {
        return <AccessDenied/>
    }



    return <> {children} </>
}


