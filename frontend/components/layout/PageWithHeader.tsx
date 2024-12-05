"use client"

import type { PropsWithChildren } from "react";

export default function PageWithHeader({ children }: PropsWithChildren) {
    return (
        <div className="flex flex-col h-screen w-screen">
            <div className="bg-black items-center text-white w-screen py-5 px-10 flex flex-row text-lg h-[80px]">
                <div className="flex flex-row items-center ml-auto uppercase font-bold italic gap-x-1.5 tracking-wide">
                    Demo-Application
                </div>
            </div>
            {children}
        </div>
    )
}
