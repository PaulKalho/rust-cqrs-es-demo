"use client";

import React, { useState } from "react";
import { useGetProjectMutation, useCreateProjectMutation } from "@/mutations/projectMutation";

type ProjectDTO = {
    project_id: string;
};

export default function Home() {
    const [projectId, setProjectId] = useState<string>("");

    // For Get Project
    const { mutate: getProject, data, isPending, error } = useGetProjectMutation();

    // For Create Project
    const { mutate: createProject, isLoading: isCreating, error: createError } = useCreateProjectMutation();

    const [newProjectId, setNewProjectId] = useState<string>("");

    const handleFetch = () => {
        if (projectId) {
            getProject(projectId); // Trigger the mutation with the project ID
        } else {
            alert("Please enter a Project ID");
        }
    };

    const handleCreateProject = () => {
        if (newProjectId) {
            const newProject: ProjectDTO = { project_id: newProjectId };
            createProject(newProject); // Create the project with the entered project ID
        } else {
            alert("Please enter a new Project ID to create");
        }
    };

    return (
        <div className="flex justify-center items-center min-h-screen bg-gray-100">
            <div className="bg-white p-8 rounded-lg shadow-lg w-full max-w-md">
                <h1 className="text-3xl font-semibold text-center text-gray-800 mb-6">
                    Fetch Project Details
                </h1>
                <div className="mb-4">
                    <input
                        type="text"
                        placeholder="Enter Project ID to fetch"
                        value={projectId}
                        onChange={(e) => setProjectId(e.target.value)} // Update projectId state
                        className="w-full p-3 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    />
                </div>
                <button
                    onClick={handleFetch}
                    disabled={isPending}
                    className={`w-full p-3 mt-4 text-white rounded-md 
                                ${isPending ? 'bg-gray-400' : 'bg-blue-600 hover:bg-blue-700'} 
                                focus:outline-none focus:ring-2 focus:ring-blue-500`}
                >
                    {isPending ? "Loading..." : "Fetch Project"}
                </button>

                {error && (
                    <div className="mt-4 text-red-600 text-center">
                        Error: {error.message}
                    </div>
                )}

                {data && (
                    <div className="mt-6 bg-gray-50 p-4 rounded-lg shadow-md">
                        <h2 className="text-2xl font-semibold text-gray-800">Project Details</h2>
                        <p className="text-lg mt-2"><strong>ID:</strong> {data.project_id}</p>
                        {/* Render additional project details as needed */}
                    </div>
                )}

                <div className="mt-8">
                    <h1 className="text-3xl font-semibold text-center text-gray-800 mb-6">
                        Create a New Project
                    </h1>
                    <div className="mb-4">
                        <input
                            type="text"
                            placeholder="Enter Project ID to create"
                            value={newProjectId}
                            onChange={(e) => setNewProjectId(e.target.value)} // Update newProjectId state
                            className="w-full p-3 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        />
                    </div>
                    <button
                        onClick={handleCreateProject}
                        disabled={isCreating}
                        className={`w-full p-3 mt-4 text-white rounded-md 
                                    ${isCreating ? 'bg-gray-400' : 'bg-green-600 hover:bg-green-700'} 
                                    focus:outline-none focus:ring-2 focus:ring-green-500`}
                    >
                        {isCreating ? "Creating..." : "Create Project"}
                    </button>

                    {createError && (
                        <div className="mt-4 text-red-600 text-center">
                            Error: {createError.message}
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
}

