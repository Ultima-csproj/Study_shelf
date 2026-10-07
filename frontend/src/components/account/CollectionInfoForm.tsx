import { useState, type FormEvent } from "react";
import type { CollectionInfo } from "../../api/personalCollections";
import styles from "./AccountForm.module.css";

interface CollectionInfoFormProps {
  info: CollectionInfo;
  busy: boolean;
  onSave: (info: CollectionInfo) => Promise<void>;
}
export function CollectionInfoForm({
  info,
  busy,
  onSave,
}: CollectionInfoFormProps) {
  const [input, setInput] = useState<CollectionInfo>({
    title: info.title,
    shortDescription: info.shortDescription,
    description: info.description,
    purpose: info.purpose,
    targetAudience: info.targetAudience,
  });
  const submit = (event: FormEvent) => {
    event.preventDefault();
    void onSave(input);
  };
  const fields: { key: keyof CollectionInfo; label: string; limit: number }[] =
    [
      { key: "title", label: "Название", limit: 160 },
      { key: "shortDescription", label: "Краткое описание", limit: 500 },
      { key: "description", label: "Описание", limit: 10000 },
      { key: "purpose", label: "Цель", limit: 2000 },
      { key: "targetAudience", label: "Для кого", limit: 2000 },
    ];
  return (
    <form className={styles.form} onSubmit={submit}>
      <fieldset disabled={busy}>
        {fields.map((field) => (
          <label key={field.key}>
            {field.label}
            <textarea
              required={field.key === "title"}
              maxLength={field.limit}
              value={input[field.key]}
              onChange={(event) =>
                setInput((current) => ({
                  ...current,
                  [field.key]: event.target.value,
                }))
              }
            />
          </label>
        ))}
        <button type="submit">Сохранить сведения</button>
      </fieldset>
    </form>
  );
}
