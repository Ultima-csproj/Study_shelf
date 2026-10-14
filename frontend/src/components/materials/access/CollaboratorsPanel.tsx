import { useCallback, useState, type FormEvent } from "react";
import {
  getCollaborators,
  setCollaborator,
  collaboratorLabels,
  type MaterialHeader,
} from "../../../api/materialWorkspace";
import { PagedPanel } from "../shared/PagedPanel";
import styles from "../shared/Panel.module.css";

interface CollaboratorsPanelProps {
  material: MaterialHeader;
  busy: boolean;
  run: (operation: () => Promise<unknown>) => Promise<boolean>;
}
export function CollaboratorsPanel({
  material,
  busy,
  run,
}: CollaboratorsPanelProps) {
  const [user, setUser] = useState("");
  const [permissions, setPermissions] = useState(["read"]);
  const [reason, setReason] = useState("");
  const load = useCallback(
    (page: number, signal: AbortSignal) =>
      getCollaborators(material.id, { page, q: "", state: "" }, signal),
    [material.id],
  );
  const submit = (event: FormEvent) => {
    event.preventDefault();
    const id = Number(user);
    if (!Number.isSafeInteger(id) || id <= 0) {
      return;
    }
    void run(() => setCollaborator(material, id, true, permissions, reason));
  };
  return (
    <section data-material-panel className={styles.card}>
      <h2>Приглашённые пользователи</h2>
      <p>
        Права действуют для аккаунта. Указание соавтора в редакции само по себе
        доступа не предоставляет.
      </p>
      <form className={styles.form} onSubmit={submit}>
        <fieldset className={styles.fields} disabled={busy}>
          <label>
            ID аккаунта
            <input
              required
              type="number"
              min={1}
              value={user}
              onChange={(event) => setUser(event.target.value)}
            />
          </label>
          <div className={styles.permissionList}>
            {Object.entries(collaboratorLabels).map(([permission, label]) => (
              <label key={permission}>
                <input
                  type="checkbox"
                  checked={permissions.includes(permission)}
                  onChange={(event) => {
                    if (event.target.checked) {
                      setPermissions((current) => [...current, permission]);
                    } else {
                      setPermissions((current) =>
                        current.filter((value) => value !== permission),
                      );
                    }
                  }}
                />
                {label}
              </label>
            ))}
          </div>
          <label>
            Причина
            <textarea
              required
              minLength={3}
              maxLength={2000}
              value={reason}
              onChange={(event) => setReason(event.target.value)}
            />
          </label>
          <button type="submit">Сохранить приглашение и разрешения</button>
        </fieldset>
      </form>
      <PagedPanel
        title="Текущие приглашения"
        load={load}
        render={(collaborator) => (
          <div className={styles.card}>
            <h3>
              {collaborator.name} · #{collaborator.userId}
            </h3>
            <p>{collaborator.active ? "Действует" : "Отозвано"}</p>
            <p>
              {collaborator.permissions
                .map(
                  (permission) => collaboratorLabels[permission] ?? permission,
                )
                .join(", ")}
            </p>
            <div className={styles.actions}>
              <button
                type="button"
                onClick={() => {
                  setUser(String(collaborator.userId));
                  setPermissions(collaborator.permissions);
                }}
              >
                Выбрать для изменения
              </button>
              {collaborator.active && (
                <button
                  type="button"
                  disabled={busy || reason.trim().length < 3}
                  onClick={() =>
                    void run(() =>
                      setCollaborator(
                        material,
                        collaborator.userId,
                        false,
                        [],
                        reason,
                      ),
                    )
                  }
                >
                  Отозвать приглашение
                </button>
              )}
            </div>
          </div>
        )}
      />
    </section>
  );
}
