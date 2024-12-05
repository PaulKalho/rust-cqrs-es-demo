"use client"

import type { InputAdornmentProps } from "@mui/material";
import type { ChangeEvent, InputHTMLAttributes } from "react";

type DateInputProps = {
    value: string;
    id?: string;
    label?: string;
    leftAdornment?: React.ReactElement<InputAdornmentProps>;
    rightAdornment?: React.ReactElement<InputAdornmentProps>;
    onChange?: (date: string) => void;
    onChangeEvent?: (event: ChangeEvent<HTMLInputElement>) => void;
} & Omit<InputHTMLAttributes<HTMLInputElement>, "id" | "value" | "label" | "onChange" | "type">;

/**
 * The state of the DateInput has to be managed by the parent
 */
export default function DateInput({
    value,
    id,
    label,
    leftAdornment,
    rightAdornment,
    onChange = () => undefined,
    onChangeEvent = () => undefined,
    ...rest
}: DateInputProps) {
    return (
        <div>
            {label && (
                <label htmlFor={id} className="mb-4 text-sm text-gray-900 font-medium">
                    {label}
                </label>
            )}
            <div className="relative mt-2 rounded-md shadow-sm">
                {leftAdornment && (
                    <span className="absolute inset-y-0 left-0 flex items-center pl-3">
                        {leftAdornment}
                    </span>
                )}
                <input
                    type="date"
                    id={id}
                    className={
                        "block w-full rounded-md border-0 py-1.5 " +
                        (leftAdornment ? "pl-11 " : "pl-5 ") +
                        "pr-20 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400"
                    }
                    value={value}
                    onChange={(e) => {
                        onChange(e.target.value);
                        onChangeEvent(e);
                    }}
                    {...rest}
                />
                {rightAdornment && (
                    <span className="absolute inset-y-0 right-0 flex items-center pr-3">
                        {rightAdornment}
                    </span>
                )}
            </div>
        </div>
    );
}

