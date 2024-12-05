import { useState } from "react";
import Input from "./inputs/Input";
import Modal, { ModalProps } from "./Modal";
import { useGetProjectMutation } from "@/mutations/projectMutation";
import { ProjectItem } from "./ProjectList";
import { useAlert } from "@/hooks/useAlert";

type SelectProjectsModalProps = Omit<ModalProps, "children">

export default function SelectProjectsModal({
    isOpen,
    onClose,
    className = ""
}: SelectProjectsModalProps) {
    const { setAlert } = useAlert()

    const [projectID, setProjectID] = useState<string>("")

    const { mutate, data, isPending } = useGetProjectMutation(setAlert)

    function getProject(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault()

        if (projectID.length > 0) {
            mutate(projectID)
        }
    }

    return (
        <Modal isOpen={isOpen} onClose={onClose} className={className}>
            <h2 className="text-xl font-bold">Auswählbare Projekte</h2>
            <div>
                {data ? <ProjectItem project={data} /> : <></>}
            </div>
            <form onSubmit={(e) => getProject(e)} className="mt-4 space-y-4 text-sm">
                <div>
                    <Input
                        type="text"
                        value={projectID}
                        label="ProjectID"
                        onChange={setProjectID}
                    />
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
                        disabled={isPending}
                    >
                        <div>
                            Get
                        </div>
                    </button>
                </div>
            </form>
        </Modal>
    )
}
