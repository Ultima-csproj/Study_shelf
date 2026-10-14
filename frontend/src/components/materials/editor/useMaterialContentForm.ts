import { useState, type FormEvent } from "react";
import type { ContentInput } from "../../../api/materialWorkspace";
import type {
  ChangeMaterialField,
  MaterialContentFormProps,
  MaterialFormFields,
  NamedCredit,
} from "./types";

interface MaterialFormOptions extends Pick<
  MaterialContentFormProps,
  "initial" | "mode" | "busy" | "onSave"
> {}

export function useMaterialContentForm({
  initial,
  mode,
  busy,
  onSave,
}: MaterialFormOptions) {
  const [fields, setFields] = useState<MaterialFormFields>(() => ({
    title: initial.title,
    description: initial.description,
    kind: initial.materialType,
    year: initial.sourceYear?.toString() ?? "",
    preview: initial.previewUrl ?? "",
    duration: initial.durationSeconds?.toString() ?? "",
    fileSize: initial.fileSizeBytes?.toString() ?? "",
    tags: initial.tags.join(", "),
    credits: initial.credits.map((credit) => ({
      ...credit,
      rowKey: crypto.randomUUID(),
    })),
  }));
  const {
    title,
    description,
    kind,
    year,
    preview,
    duration,
    fileSize,
    tags,
    credits,
  } = fields;
  const [reason, setReason] = useState("");
  const [error, setError] = useState<string | null>(null);
  const isNewVersion = mode === "new_revision";
  const changeField: ChangeMaterialField = (field, value) => {
    setFields((current) => ({ ...current, [field]: value }));
  };
  const updateCredit = (index: number, value: NamedCredit) => {
    setFields((current) => ({
      ...current,
      credits: current.credits.map((credit, position) =>
        position === index ? value : credit,
      ),
    }));
  };
  const addCredit = () => {
    setFields((current) => ({
      ...current,
      credits: [
        ...current.credits,
        { contributorId: 0, role: "co_author", rowKey: crypto.randomUUID() },
      ],
    }));
  };
  const removeCredit = (index: number) => {
    setFields((current) => ({
      ...current,
      credits: current.credits.filter((_, position) => position !== index),
    }));
  };
  const submit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (busy) {
      return;
    }
    if (credits.some((credit) => credit.contributorId <= 0)) {
      setError(
        "Выберите каждого участника из результатов поиска или уберите незаполненную строку.",
      );
      return;
    }
    const identities = new Set(
      credits.map((credit) => `${credit.contributorId}:${credit.role}`),
    );
    if (identities.size !== credits.length) {
      setError("Один участник с одинаковой ролью указан несколько раз.");
      return;
    }
    setError(null);
    const button = (event.nativeEvent as SubmitEvent).submitter;
    const send =
      button instanceof HTMLButtonElement && button.value === "submit";
    const content: ContentInput = {
      title,
      description,
      materialType: kind,
      sourceYear: year === "" ? null : Number(year),
      previewUrl: preview.trim() || null,
      durationSeconds: duration === "" ? null : Number(duration),
      fileSizeBytes: fileSize === "" ? null : Number(fileSize),
      tags: tags
        .split(",")
        .map((tag) => tag.trim())
        .filter(Boolean),
      credits: credits.map((credit) => ({
        contributorId: credit.contributorId,
        role: credit.role,
      })),
    };
    let cause = "Сохранение черновика";
    if (mode === "create") {
      cause = "Первоначальное создание материала";
    } else if (isNewVersion) {
      cause = reason;
    }
    void onSave(content, send, cause);
  };

  return {
    fields,
    reason,
    error,
    changeField,
    setReason,
    updateCredit,
    addCredit,
    removeCredit,
    submit,
  };
}
