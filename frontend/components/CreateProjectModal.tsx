import { useState } from "react"
import { useCreateProjectMutation } from "@/mutations/projectMutation";
import Modal, { type ModalProps } from "./Modal"
import React from "react";
import Input from "./inputs/Input";
import DateInput from "./inputs/DateInput";
import { useAlert } from "@/hooks/useAlert";

type CreateProjectModalProps = Omit<ModalProps, "children">;

export default function CreateProjectModal({
    isOpen,
    onClose,
    className = "",
}: CreateProjectModalProps) {
    const { setAlert } = useAlert()

    const [projectName, setProjectName] = useState<string>("")
    const [projectStart, setProjectStart] = useState<string>("")
    const [projectEnd, setProjectEnd] = useState<string>("")

    const { mutate, isPending: loading } = useCreateProjectMutation(setAlert)

    function createProject(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault()

        if (projectName.length > 0 && projectStart.length > 0 && projectEnd.length > 0) {
            mutate({
                project_name: projectName,
                project_start: new Date(projectStart),
                project_end: new Date(projectEnd),
                participants_name: [],
            })
            onClose()
        }
    }

    return (
        <Modal isOpen={isOpen} onClose={onClose} className={className}>
            <h2 className="text-xl font-bold">Projekte:</h2>
            <form onSubmit={(e) => createProject(e)} className="mt-4 space-y-4 text-sm">
                <div>
                    <Input
                        type="text"
                        value={projectName}
                        label="Project Name"
                        onChange={setProjectName}
                    />
                </div>
                <div>
                    <DateInput
                        value={projectStart}
                        label="Project Start"
                        id="date-input"
                        onChange={setProjectStart} />
                </div>
                <div>
                    <DateInput
                        value={projectEnd}
                        label="Project Ende"
                        id="date-input-end"
                        onChange={setProjectEnd} />
                </div>
                <div className="flex justify-end">
                    <button
                        type="button"
                        onClick={onClose}
                        className="w-[78px] h-[36px] px-4 py-2 mr-2 text-gray-700 bg-gray-100 rounded hover:bg-gray-200"
                    >
                        Cancel
                    </button>
                    <button
                        type="submit"
                        className="flex flex-col w-[78px] h-[36px] px-4 py-2 text-white bg-blue-500 rounded hover:bg-blue-600"
                        disabled={loading}
                    >
                        <div>
                            Create
                        </div>
                    </button>
                </div>
            </form>
        </Modal>
    )
}
