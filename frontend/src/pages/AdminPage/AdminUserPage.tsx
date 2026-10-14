import { useCallback, useState } from "react";
import { Link, useParams } from "react-router-dom";
import { getUser, stateLabels, memberLabels } from "../../api/admin";
import { useApiResource } from "../../hooks/useApiResource";
import { useViewer } from "../../hooks/useViewer";
import { UserEditor } from "../../components/admin/UserEditor";
import { useAdminAccess } from "../../hooks/useAdminAccess";
import type { RunAction } from "../../components/admin/AdminList";
import styles from "../../components/admin/Admin.module.css";
export function AdminUserPage() {
  const id = Number(useParams().id);
  const access = useAdminAccess();
  const { refreshViewer } = useViewer();
  const [revision, setRevision] = useState(0);
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const load = useCallback(
    (signal: AbortSignal) => {
      void revision;
      return getUser(id, signal);
    },
    [id, revision],
  );
  const { result, loading, error, retry } = useApiResource(
    load,
    "Не удалось загрузить аккаунт",
  );
  const run: RunAction = async (operation) => {
    if (busy) {
      return false;
    }
    setBusy(true);
    setNotice(null);
    try {
      await operation();
      setRevision((r) => r + 1);
      setNotice("Изменения сохранены.");
      await refreshViewer();
      return true;
    } catch (reason) {
      setNotice(
        reason instanceof Error
          ? reason.message
          : "Не удалось изменить аккаунт",
      );
      return false;
    } finally {
      setBusy(false);
    }
  };
  return (
    <section>
      <Link to="/admin/users">← Аккаунты</Link>
      <h1>{result?.user.name ?? "Аккаунт"}</h1>
      {loading && <p role="status">Загружаем…</p>}
      {(error || notice) && (
        <p role="status">
          {error ?? notice}{" "}
          <button type="button" onClick={retry}>
            Обновить
          </button>
        </p>
      )}
      {result && (
        <article className={styles.card}>
          <p>
            #{result.user.id} · {stateLabels[result.user.state]}
          </p>
          {result.user.email !== null ? (
            <p>Email: {result.user.email}</p>
          ) : (
            <p>Email недоступен вашему набору полномочий.</p>
          )}
          <h2>Членство</h2>
          <ul>
            {result.memberships.map((m) => (
              <li key={m.institutionId}>
                {m.institutionName} — {stateLabels[m.state] ?? m.state};{" "}
                {m.roles.join(", ")} ·{" "}
                {m.permissions.map((p) => memberLabels[p] ?? p).join(", ")}
              </li>
            ))}
          </ul>
          <h2>Связанные авторы</h2>
          <ul>
            {result.contributors.map((c) => (
              <li key={c.id}>
                #{c.id} · {c.name}
              </li>
            ))}
          </ul>
          <UserEditor
            key={result.user.version}
            user={result.user}
            access={access}
            run={run}
            busy={busy}
          />
        </article>
      )}
    </section>
  );
}
