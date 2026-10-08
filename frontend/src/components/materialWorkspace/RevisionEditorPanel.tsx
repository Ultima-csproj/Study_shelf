import {
  saveRevision,
  transitionRevision,
  forkRevision,
  type ContentInput,
  type RevisionResponse,
  type Workspace,
} from "../../api/materialWorkspace";
import { MaterialContentForm } from "./MaterialContentForm";
import { RevisionContent } from "./RevisionContent";
import styles from "./Workspace.module.css";

interface RevisionEditorPanelProps {
  value: RevisionResponse;
  workspace: Workspace;
  reason: string;
  action: {
    busy: boolean;
    run: (operation: () => Promise<unknown>) => Promise<boolean>;
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
  const save = async (
    content: ContentInput,
    submit: boolean,
    cause: string,
  ) => {
    const revision = value.revision;
    let saved = false;
    const success = await action.run(async () => {
      const response = await saveRevision(revision, content, cause);
      saved = true;
      if (submit) {
        await transitionRevision(
          { ...revision, version: response.version },
          "submit",
          cause,
        );
      }
    });
    if (!success && saved) {
      changed();
    }
  };
  const reconsider = async () => {
    const material = workspace.material;
    const revision = value.revision;
    let discarded = false;
    const success = await action.run(async () => {
      await transitionRevision(revision, "discard", reason);
      discarded = true;
      const result = await forkRevision(material, revision.id, reason);
      onChoose(result.revisionId);
    });
    if (!success && discarded) {
      changed();
    }
  };
  const ready = !action.busy && reason.trim().length >= 3;
  return (
    <>
      <RevisionContent value={value} />
      {value.revision.state === "draft" && workspace.capabilities.edit && (
        <MaterialContentForm
          key={value.revision.id + ":" + value.revision.version}
          initial={value.revision}
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
                  transitionRevision(value.revision, "submit", reason),
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
                  transitionRevision(value.revision, "withdraw", reason),
                )
              }
            >
              Отозвать для редактирования
            </button>
          )}
        {value.revision.state === "draft" && workspace.capabilities.edit && (
          <button
            type="button"
            disabled={!ready}
            onClick={() =>
              void action.run(() =>
                transitionRevision(value.revision, "discard", reason),
              )
            }
          >
            Отменить черновик с сохранением истории
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
      {value.outdated && (
        <p>
          Текст копируется без слияния. Сверьте его с текущей публикацией и
          пересмотрите изменения перед отправкой.
        </p>
      )}
    </>
  );
}
