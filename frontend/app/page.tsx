"use client";

import React, { useState } from "react";
import PageWithHeader from "@/components/layout/PageWithHeader";
import { ProjectList } from "@/components/ProjectList";
import SelectProjectsModal from "@/components/SelectProjects";

export default function Home() {
    const [isSelectProjectsOpen, setIsSelectProjectsOpen] = useState<boolean>(false)

    return (
        <PageWithHeader>
            <div className="p-5">
                <ProjectList />
            </div>
            <div>
                <button onClick={() => { setIsSelectProjectsOpen(true) }} className="w-full max-w-sm mx-auto px-6 py-3 bg-blue-600 text-white font-semibold text-lg rounded-lg shadow hover:bg-blue-700 transition duration-200">
                    DEMO: Open Select Projects Modal
                </button>
            </div>
            <SelectProjectsModal isOpen={isSelectProjectsOpen} onClose={() => setIsSelectProjectsOpen(false)} />
        </PageWithHeader>
    );
}
