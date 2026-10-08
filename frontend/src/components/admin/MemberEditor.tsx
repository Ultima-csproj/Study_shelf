import { useState, type FormEvent } from "react";
import {
  memberLabels,
  stateLabels,
  setMembership,
  decideMembership,
  type Membership,
} from "../../api/admin";
import type { RunAction } from "./AdminList";
import { Permissions } from "./Permissions";
import styles from "./Admin.module.css";
interface MemberEditorProps {
  member: Membership;
  editable: boolean;
  initial: boolean;
  run: RunAction;
}
export function MemberEditor({
  member,
  editable,
  initial,
  run,
}: MemberEditorProps) {
  const [permissions, setPermissions] = useState(
    initial ? Object.keys(memberLabels) : member.permissions,
  );
  const [roles, setRoles] = useState(member.roles);
  const [state, setState] = useState(
    initial || member.state === "active" ? "active" : "revoked",
  );
  const [role, setRole] = useState("student");
  const [reason, setReason] = useState("");
  const submit = (event: FormEvent) => {
    event.preventDefault();
    void run(() =>
      setMembership(member.institutionId, member.userId, {
        expectedVersion: member.version,
        state,
        roles,
        permissions,
        reason,
      }),
    );
  };
  return (
    <article className={styles.card}>
      <h3>
        {member.userName} · #{member.userId}
      </h3>
      <p>{stateLabels[member.state] ?? member.state}</p>
      {editable && (
        <div className={styles.form}>
          <label>
            Причина
            <textarea
              minLength={3}
              maxLength={2000}
              value={reason}
              onChange={(event) => setReason(event.target.value)}
            />
          </label>
          {member.state === "pending" && !initial ? (
            <>
              <label>
                Подтверждаемый статус
                <select
                  value={role}
                  onChange={(event) => setRole(event.target.value)}
                >
                  <option value="student">Студент</option>
                  <option value="teacher">Преподаватель</option>
                </select>
              </label>
              <div className={styles.actions}>
                <button
                  type="button"
                  disabled={reason.trim().length < 3}
                  onClick={() =>
                    void run(() => decideMembership(member, true, role, reason))
                  }
                >
                  Принять заявку
                </button>
                <button
                  type="button"
                  disabled={reason.trim().length < 3}
                  onClick={() =>
                    void run(() =>
                      decideMembership(member, false, role, reason),
                    )
                  }
                >
                  Отклонить
                </button>
              </div>
            </>
          ) : (
            <form className={styles.form} onSubmit={submit}>
              <label>
                Членство
                <select
                  value={state}
                  onChange={(event) => setState(event.target.value)}
                >
                  <option value="active">Активное</option>
                  <option value="revoked">Отозвано</option>
                </select>
              </label>
              <Permissions
                legend="Учебные статусы"
                labels={{ student: "Студент", teacher: "Преподаватель" }}
                value={roles}
                onChange={setRoles}
              />
              <Permissions
                legend="Разрешения учреждения"
                labels={memberLabels}
                value={permissions}
                onChange={setPermissions}
              />
              <button type="submit" disabled={reason.trim().length < 3}>
                Сохранить членство
              </button>
            </form>
          )}
        </div>
      )}
    </article>
  );
}
