import { useMutation } from "@tanstack/react-query";
import { getConfig } from "@/config/config";

const PROJECTS_ENDPOINT = "project/"
const config = getConfig()


type ProjectDTO = {
    project_id: string
}

function useGetProjectMutation() {
    return useMutation({
        mutationFn: async function getProject(project_id: string) {
            if (!project_id) {
                throw new Error("Project ID is required.")
            }

            const response = await fetch(`${config.apiUrl}${PROJECTS_ENDPOINT}${project_id}`)
            if (!response.ok) {
                throw new Error(`Error fetching project: ${response.statusText}`)
            }

            const data = await response.json()
            return data;
        },
        onError: (error) => {
            console.error("Get Project failed:", error)
        },
        onSuccess: (data) => {
            console.log("Got Project successfully:", data)
        }
    })
}

function useCreateProjectMutation() {
    return useMutation({
        mutationFn: async function createProject(project: ProjectDTO) {
            console.log(project)
            const response = await fetch(`${config.apiUrl}${PROJECTS_ENDPOINT}${project.project_id}`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(project)
            });

            if (!response.ok) {
                throw new Error(`Error creating projects: ${response.statusText}`)
            }
        },
        onError: (error) => {
            console.error("Project creation failed:", error)
        },
        onSuccess: (data) => {
            console.log("Project created successfully:", data)
        }
    })
}

export {
    useGetProjectMutation,
    useCreateProjectMutation
}
