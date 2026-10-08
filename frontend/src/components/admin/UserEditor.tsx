import { useState, type FormEvent } from "react";
import {
  platformLabels,
  setPlatformPermissions,
  setUserState,
  revokeUserSessions,
  type AdminUser,
  type AdminAccess,
} from "../../api/admin";
import { Permissions } from "./Permissions";
import type { RunAction } from "./AdminList";
import styles from "./Admin.module.css";
interface UserEditorProps {
  user: AdminUser;
  access: AdminAccess;
  run: RunAction;
  busy: boolean;
}
export function UserEditor({ user, access, run, busy }: UserEditorProps) {
  const [permissions, setPermissions] = useState(user.permissions);
  const [reason, setReason] = useState("");
  const manage = access.permissions.includes("manage_accounts");
  const grant = access.permissions.includes("manage_platform_permissions");
  const submit = (event: FormEvent) => {
    event.preventDefault();
    void run(() => setPlatformPermissions(user, permissions, reason));
  };
  return (
    <fieldset disabled={busy} className={styles.fieldset}>
      <label>
        Причина административного действия
        <textarea
          required
          minLength={3}
          maxLength={2000}
          value={reason}
          onChange={(event) => setReason(event.target.value)}
        />
      </label>
      {manage && (
        <div className={styles.actions}>
          <button
            type="button"
            disabled={reason.trim().length < 3}
            onClick={() =>
              void run(() =>
                setUserState(user, user.state !== "active", reason),
              )
            }
          >
            {user.state === "active" ? "Отключить аккаунт" : "Включить аккаунт"}
          </button>
          <button
            type="button"
            disabled={reason.trim().length < 3}
            onClick={() => void run(() => revokeUserSessions(user.id, reason))}
          >
            Завершить все сессии
          </button>
        </div>
      )}
      {grant && (
        <form className={styles.form} onSubmit={submit}>
          <Permissions
            legend="Глобальные разрешения"
            labels={platformLabels}
            value={permissions}
            onChange={setPermissions}
          />
          <button type="submit" disabled={reason.trim().length < 3}>
            Сохранить разрешения
          </button>
        </form>
      )}
    </fieldset>
  );
}
