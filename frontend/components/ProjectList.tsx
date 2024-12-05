"use client"

import { useState } from "react";
import { v4 } from "uuid";
import CreateProjectModal from "./CreateProjectModal";

export type Project = {
    project_id: string;
    project_name: string;
    project_start: Date;
    project_end: Date;
    participants_name: string[];
}

type ProjectItemProps = {
    project: Project
}

function ProjectItem({ project }: ProjectItemProps) {
    return (
        <div className="w-full mx-auto border rounded-lg shadow-lg p-6 bg-white mt-5">
            <h2 className="text-xl font-bold mb-3 text-gray-800">{project.project_name}</h2>
            <div className="text-gray-600">
                <p>
                    <span className="font-semibold">Start Date:</span>{" "}
                    {project.project_start.toDateString()}
                </p>
                <p>
                    <span className="font-semibold">End Date:</span>{" "}
                    {project.project_end.toDateString()}
                </p>
            </div>
        </div>
    );
}

function ProjectList() {
    const [createProjectOpen, setCreateProjectOpen] = useState<boolean>(false)

    // TODO: getProjects
    const mockProject: Project = {
        project_id: v4(),
        project_name: "Mock",
        project_start: new Date(),
        project_end: new Date(),
        participants_name: ["test"]
    }


    return (
        <div>
            <b>Projects:</b>
            {/* TODO: for project in projects */}
            <ProjectItem project={mockProject} />

            <div className="mt-6">
                <button onClick={() => { setCreateProjectOpen(true) }} className="w-full max-w-sm mx-auto px-6 py-3 bg-blue-600 text-white font-semibold text-lg rounded-lg shadow hover:bg-blue-700 transition duration-200">
                    Import Projects
                </button>
                <CreateProjectModal onClose={() => { setCreateProjectOpen(false) }} isOpen={createProjectOpen} />
            </div>
        </div>
    )
}

export {
    ProjectItem,
    ProjectList
}
