import { useState } from "react";
import {
  saveRevision,
  transitionRevision,
  forkRevision,
  type ContentInput,
  type RevisionResponse,
  type Workspace,
} from "../../../api/materialWorkspace";
import { MaterialContentForm } from "../editor/MaterialContentForm";
import { RevisionContent } from "./RevisionContent";
import styles from "../shared/Panel.module.css";

interface RevisionEditorPanelProps {
  value: RevisionResponse;
  workspace: Workspace;
  reason: string;
  action: {
    busy: boolean;
    run: (
      operation: () => Promise<unknown>,
      successMessage?: string,
    ) => Promise<boolean>;
  };
  changed: () => void;
  onChoose: (revision: number) => void;
}

export function RevisionEditorPanel({
  value,
  workspace,
  reason,
  action,
  changed,
  onChoose,
}: RevisionEditorPanelProps) {
  const [confirmDiscard, setConfirmDiscard] = useState(false);
  const save = async (
    content: ContentInput,
    submit: boolean,
    cause: string,
  ) => {
    const revision = value.revision;
    let saved = false;
    const success = await action.run(
      async () => {
        const response = await saveRevision(revision, content, cause);
        saved = true;
        if (submit) {
          try {
            await transitionRevision(
              { ...revision, version: response.version },
              "submit",
              cause,
            );
          } catch (error) {
            const message =
              error instanceof Error ? error.message : "Попробуйте ещё раз";
            throw new Error(
              `Черновик сохранён, но отправить его на модерацию не удалось: ${message}`,
            );
          }
        }
      },
      submit
        ? "Материал отправлен на модерацию. Он станет публичным после одобрения."
        : "Черновик сохранён.",
    );
    if (!success && saved) {
      changed();
    }
  };
  const reconsider = async () => {
    const material = workspace.material;
    const revision = value.revision;
    let discarded = false;
    const success = await action.run(async () => {
      await transitionRevision(
        revision,
        "discard",
        "Пересмотр черновика на актуальной базе",
      );
      discarded = true;
      const result = await forkRevision(
        material,
        revision.id,
        "Пересмотр черновика на актуальной базе",
      );
      onChoose(result.revisionId);
    });
    if (!success && discarded) {
      changed();
    }
  };
  const ready = !action.busy;
  const transitionReason =
    reason.trim().length >= 3 ? reason : "Действие автора над рабочей версией";
  return (
    <>
      {(value.revision.state !== "draft" || !workspace.capabilities.edit) && (
        <RevisionContent value={value} />
      )}
      {value.revision.state === "draft" && workspace.capabilities.edit && (
        <MaterialContentForm
          key={value.revision.id + ":" + value.revision.version}
          initial={value.revision}
          mode={
            value.revision.baseRevisionId === null
              ? "edit_draft"
              : "new_revision"
          }
          busy={action.busy}
          allowSubmit={workspace.capabilities.submit && !value.outdated}
          onSave={save}
        />
      )}
      <div className={styles.actions}>
        {value.revision.state === "draft" &&
          workspace.capabilities.submit &&
          !workspace.capabilities.edit && (
            <button
              type="button"
              disabled={!ready || value.outdated}
              onClick={() =>
                void action.run(() =>
                  transitionRevision(
                    value.revision,
                    "submit",
                    transitionReason,
                  ),
                )
              }
            >
              Отправить готовую редакцию на модерацию
            </button>
          )}
        {value.revision.state === "pending_moderation" &&
          workspace.capabilities.edit && (
            <button
              type="button"
              disabled={!ready}
              onClick={() =>
                void action.run(() =>
                  transitionRevision(
                    value.revision,
                    "withdraw",
                    transitionReason,
                  ),
                )
              }
            >
              Отозвать для редактирования
            </button>
          )}
        {value.revision.state === "draft" && workspace.capabilities.edit && (
          <button
            type="button"
            disabled={action.busy}
            onClick={() => setConfirmDiscard(true)}
          >
            Отменить изменения
          </button>
        )}
        {value.outdated &&
          value.revision.state === "draft" &&
          workspace.capabilities.edit && (
            <button
              type="button"
              disabled={!ready}
              onClick={() => void reconsider()}
            >
              Создать новый черновик на актуальной базе
            </button>
          )}
      </div>
      {confirmDiscard && (
        <section
          className={styles.banner}
          aria-label="Подтверждение отмены изменений"
        >
          <h3>Отменить этот черновик?</h3>
          <p>
            Продолжить его редактирование будет нельзя. История сохранится
            {workspace.material.publishedRevisionId !== null
              ? ", текущая публикация останется доступной"
              : ""}
            .
          </p>
          <div className={styles.actions}>
            <button
              type="button"
              disabled={action.busy}
              onClick={() =>
                void action
                  .run(
                    () =>
                      transitionRevision(
                        value.revision,
                        "discard",
                        "Автор отменил изменения",
                      ),
                    "Черновик отменён. История сохранена.",
                  )
                  .then((success) => {
                    if (success) {
                      setConfirmDiscard(false);
                    }
                  })
              }
            >
              Да, отменить изменения
            </button>
            <button
              type="button"
              disabled={action.busy}
              onClick={() => setConfirmDiscard(false)}
            >
              Продолжить работу
            </button>
          </div>
        </section>
      )}
      {value.outdated && (
        <p>
          Текст копируется без слияния. Сверьте его с текущей публикацией и
          пересмотрите изменения перед отправкой.
        </p>
      )}
    </>
  );
}
