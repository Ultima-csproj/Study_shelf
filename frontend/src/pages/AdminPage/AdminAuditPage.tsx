import { getAudit } from "../../api/admin";
import { AdminList } from "../../components/admin/AdminList";
import styles from "../../components/admin/Admin.module.css";
const actions: Record<string, string> = {
  set_user_state: "Изменение состояния аккаунта",
  revoke_user_sessions: "Завершение сессий",
  set_platform_permissions: "Изменение глобальных прав",
  set_membership: "Изменение членства",
  request_membership: "Личная заявка на вступление",
  rename_institution: "Изменение учреждения",
  decide_contributor_link: "Решение по привязке автора",
  decide_institution_registration: "Решение по регистрации учреждения",
};
export function AdminAuditPage() {
  return (
    <AdminList
      title="Журнал действий"
      load={getAudit}
      render={(a) => (
        <article className={styles.card}>
          <h2>{actions[a.action] ?? a.action}</h2>
          <p>
            {a.createdAt} ·{" "}
            {a.actorName ?? "Локальная административная команда"}
          </p>
          <p>
            {a.targetKind} #{a.targetId}
            {a.institutionId !== null && ` · Учреждение #${a.institutionId}`}
          </p>
          <p>{a.reason}</p>
          <details>
            <summary>Что изменилось</summary>
            <pre className={styles.details}>
              {JSON.stringify(
                { before: a.beforeData, after: a.afterData },
                null,
                2,
              )}
            </pre>
          </details>
        </article>
      )}
    />
  );
}
