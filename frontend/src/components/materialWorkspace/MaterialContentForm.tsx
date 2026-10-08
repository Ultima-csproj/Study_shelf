import { useState, type FormEvent } from "react";
import {
  creditLabels,
  type ContentInput,
  type CreditInput,
} from "../../api/materialWorkspace";
import type { ContributorRole, MaterialType } from "../../types/material";
import styles from "./Workspace.module.css";

interface MaterialContentFormProps {
  initial: ContentInput;
  busy: boolean;
  allowSubmit: boolean;
  onSave: (
    content: ContentInput,
    submit: boolean,
    reason: string,
  ) => Promise<unknown>;
}
export function MaterialContentForm({
  initial,
  busy,
  allowSubmit,
  onSave,
}: MaterialContentFormProps) {
  const [title, setTitle] = useState(initial.title);
  const [description, setDescription] = useState(initial.description);
  const [kind, setKind] = useState(initial.materialType);
  const [year, setYear] = useState(initial.sourceYear?.toString() ?? "");
  const [preview, setPreview] = useState(initial.previewUrl ?? "");
  const [duration, setDuration] = useState(
    initial.durationSeconds?.toString() ?? "",
  );
  const [fileSize, setFileSize] = useState(
    initial.fileSizeBytes?.toString() ?? "",
  );
  const [tags, setTags] = useState(initial.tags.join(", "));
  const [credits, setCredits] = useState<CreditInput[]>(initial.credits);
  const [reason, setReason] = useState("Подготовка редакции материала");
  const updateCredit = (index: number, value: CreditInput) => {
    setCredits((current) =>
      current.map((credit, position) => (position === index ? value : credit)),
    );
  };
  const submit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
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
      credits,
    };
    void onSave(content, send, reason);
  };
  return (
    <form className={styles.form} onSubmit={submit}>
      <fieldset disabled={busy} className={styles.fields}>
        <label>
          Название
          <input
            required
            maxLength={300}
            value={title}
            onChange={(event) => setTitle(event.target.value)}
          />
        </label>
        <label>
          Тип материала
          <select
            value={kind}
            onChange={(event) => setKind(event.target.value as MaterialType)}
          >
            <option value="article">Статья</option>
            <option value="lecture">Лекция</option>
            <option value="book">Книга</option>
            <option value="lab">Лабораторная работа</option>
            <option value="video">Видео</option>
          </select>
        </label>
        <label>
          Текст и описание
          <textarea
            rows={12}
            maxLength={20000}
            value={description}
            onChange={(event) => setDescription(event.target.value)}
          />
        </label>
        <label>
          Год источника
          <input
            type="number"
            min={1}
            max={9999}
            value={year}
            onChange={(event) => setYear(event.target.value)}
          />
        </label>
        <label>
          Теги через запятую
          <input
            maxLength={8100}
            value={tags}
            onChange={(event) => setTags(event.target.value)}
          />
        </label>
        <fieldset className={styles.fields}>
          <legend>Участники материала</legend>
          <p>
            Авторство указывается отдельно от приглашений и прав редактирования.
          </p>
          {credits.map((credit, index) => (
            <div key={index} className={styles.credit}>
              <label>
                ID автора
                <input
                  required
                  type="number"
                  min={1}
                  value={credit.contributorId || ""}
                  onChange={(event) =>
                    updateCredit(index, {
                      ...credit,
                      contributorId: Number(event.target.value),
                    })
                  }
                />
              </label>
              <label>
                Участие
                <select
                  value={credit.role}
                  onChange={(event) =>
                    updateCredit(index, {
                      ...credit,
                      role: event.target.value as ContributorRole,
                    })
                  }
                >
                  {Object.entries(creditLabels).map(([value, label]) => (
                    <option key={value} value={value}>
                      {label}
                    </option>
                  ))}
                </select>
              </label>
              <button
                type="button"
                onClick={() =>
                  setCredits((current) =>
                    current.filter((_, position) => position !== index),
                  )
                }
              >
                Убрать
              </button>
            </div>
          ))}
          <button
            type="button"
            disabled={credits.length >= 100}
            onClick={() =>
              setCredits((current) => [
                ...current,
                { contributorId: 0, role: "co_author" },
              ])
            }
          >
            Добавить участника
          </button>
          {credits.length === 0 && (
            <p>Будет указан ваш основной автор из профиля.</p>
          )}
        </fieldset>
        <details>
          <summary>Дополнительные сведения</summary>
          <div className={styles.fields}>
            <label>
              URL обложки или превью
              <input
                type="url"
                value={preview}
                onChange={(event) => setPreview(event.target.value)}
              />
            </label>
            <label>
              Продолжительность, секунд
              <input
                type="number"
                min={0}
                max={4294967295}
                value={duration}
                onChange={(event) => setDuration(event.target.value)}
              />
            </label>
            <label>
              Размер существующего файла, байт
              <input
                type="number"
                min={0}
                max={Number.MAX_SAFE_INTEGER}
                value={fileSize}
                onChange={(event) => setFileSize(event.target.value)}
              />
            </label>
          </div>
        </details>
        <label>
          Причина изменения
          <textarea
            required
            minLength={3}
            maxLength={2000}
            value={reason}
            onChange={(event) => setReason(event.target.value)}
          />
        </label>
        <div className={styles.actions}>
          <button type="submit" value="save">
            Сохранить черновик
          </button>
          {allowSubmit && (
            <button type="submit" value="submit">
              Сохранить и отправить на публичную публикацию
            </button>
          )}
        </div>
      </fieldset>
    </form>
  );
}
