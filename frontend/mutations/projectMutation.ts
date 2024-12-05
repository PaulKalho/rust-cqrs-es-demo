import { useMutation } from "@tanstack/react-query";
import { getConfig } from "@/config/config";
import { AlertType, SetAlertType } from "@/hooks/useAlert";
import Alert from "@/components/Alert";
import { Project } from "@/components/ProjectList";

const PROJECTS_ENDPOINT = "project/"
const config = getConfig()


type ProjectDTO = {
    project_name: string;
    project_start: Date;
    project_end: Date;
    participants_name: string[];
}

function useGetProjectMutation(setAlert: SetAlertType) {
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
            // TODO: This can be done better
            const returnVal: Project = {
                project_id: data.project_id,
                project_name: data.project_name,
                project_start: new Date(data.project_start),
                project_end: new Date(data.project_end),
                participants_name: data.participants_name
            }
            return returnVal;
        },
        onError: (error) => {
            console.error("Get Project failed:", error)
            setAlert("Get Project Failed", AlertType.ERROR)
        },
        onSuccess: (data) => {
            console.log("Got Project successfully:", data)
        }
    })
}

function useCreateProjectMutation(setAlert: SetAlertType) {
    return useMutation({
        mutationFn: async function createProject(project: ProjectDTO) {
            console.log(project)
            const response = await fetch(`${config.apiUrl}${PROJECTS_ENDPOINT}`, {
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
            setAlert("Creating Project Failed", AlertType.ERROR)
        },
        onSuccess: () => {
            console.log("Project created successfully.")
            setAlert("Successfully created project", AlertType.SUCCESS)
        }
    })
}

export {
    useGetProjectMutation,
    useCreateProjectMutation
}
