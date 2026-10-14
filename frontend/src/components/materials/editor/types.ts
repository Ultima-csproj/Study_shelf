import type { ReactNode } from "react";
import type { ContentInput, CreditInput } from "../../../api/materialWorkspace";
import type { MaterialType } from "../../../types/material";

export type ContentFormMode = "create" | "edit_draft" | "new_revision";
export type NamedCredit = CreditInput & {
  displayName?: string;
  rowKey: string;
};
export interface MaterialContentFormProps {
  initial: ContentInput;
  mode: ContentFormMode;
  busy: boolean;
  allowSubmit: boolean;
  ownerField?: ReactNode;
  onSave: (
    content: ContentInput,
    submit: boolean,
    reason: string,
  ) => Promise<unknown>;
}
export interface MaterialFormFields {
  title: string;
  description: string;
  kind: MaterialType;
  year: string;
  preview: string;
  duration: string;
  fileSize: string;
  tags: string;
  credits: NamedCredit[];
}
export type ChangeMaterialField = <Key extends keyof MaterialFormFields>(
  field: Key,
  value: MaterialFormFields[Key],
) => void;
