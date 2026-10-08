import { useCallback, useState, type FormEvent } from "react";
import { Link, useParams, useSearchParams } from "react-router-dom";
import {
  getInstitution,
  getMembers,
  renameInstitution,
  setMembership,
  memberLabels,
  stateLabels,
  type Institution,
  type AdminQuery,
} from "../../api/admin";
import { useApiResource } from "../../hooks/useApiResource";
import { useViewer } from "../../hooks/useViewer";
import { useAdminAccess } from "../../hooks/useAdminAccess";
import { AdminList } from "../../components/admin/AdminList";
import { MemberEditor } from "../../components/admin/MemberEditor";
import styles from "../../components/admin/Admin.module.css";

export function AdminInstitutionPage() {
  const id = Number(useParams().id);
  const access = useAdminAccess();
  const [params] = useSearchParams();
  const section = params.get("section") ?? "members";
  const [revision, setRevision] = useState(0);
  const load = useCallback(
    (signal: AbortSignal) => {
      void revision;
      return getInstitution(id, signal);
    },
    [id, revision],
  );
  const loadMembers = useCallback(
    (p: AdminQuery, signal: AbortSignal) =>
      getMembers(
        id,
        { ...p, state: section === "requests" ? "pending" : p.state },
        signal,
      ),
    [id, section],
  );
  const changed = useCallback(() => setRevision((r) => r + 1), []);
  const { result, loading, error, retry } = useApiResource(
    load,
    "Не удалось загрузить учреждение",
  );
  const memberManager = access.institutions.some(
    (i) => i.id === id && i.permissions.includes("manage_members"),
  );
  const global = access.permissions.includes("manage_institutions");
  return (
    <section>
      <Link to="/admin/institutions">← Учреждения</Link>
      <h1>{result?.name ?? "Учреждение"}</h1>
      {loading && <p role="status">Загружаем…</p>}
      {error && (
        <p role="alert">
          {error}{" "}
          <button type="button" onClick={retry}>
            Повторить
          </button>
        </p>
      )}
      {result && (
        <>
          <p>
            Участников: {result.memberCount} · Управляющих:{" "}
            {result.managerCount}
          </p>
          <nav className={styles.actions} aria-label="Управление учреждением">
            <Link
              aria-current={section === "members" ? "page" : undefined}
              to={`/admin/institutions/${id}?section=members`}
            >
              Участники
            </Link>
            <Link
              aria-current={section === "requests" ? "page" : undefined}
              to={`/admin/institutions/${id}?section=requests&state=pending`}
            >
              Заявки на вступление
            </Link>
            {global && (
              <Link
                aria-current={section === "settings" ? "page" : undefined}
                to={`/admin/institutions/${id}?section=settings`}
              >
                Настройки учреждения
              </Link>
            )}
          </nav>
          {global && section === "settings" && (
            <InstitutionControls
              key={result.version}
              institution={result}
              onChanged={changed}
            />
          )}
          {!memberManager && (
            <p>
              Управление членством требует прав этого учреждения. Администратор
              платформы может назначить первого управляющего, если действующих
              управляющих нет.
            </p>
          )}
          {section !== "settings" && (
            <AdminList
              title={
                section === "requests"
                  ? "Заявки на вступление"
                  : "Участники учреждения"
              }
              heading="h2"
              load={loadMembers}
              states={
                section === "requests"
                  ? []
                  : ["active", "pending", "rejected", "revoked"]
              }
              stateLabels={stateLabels}
              onChanged={changed}
              render={(m, run) => (
                <MemberEditor
                  key={`${m.userId}:${m.version}`}
                  member={m}
                  editable={
                    memberManager || (global && result.managerCount === 0)
                  }
                  initial={
                    !memberManager && global && result.managerCount === 0
                  }
                  run={run}
                />
              )}
            />
          )}
        </>
      )}
    </section>
  );
}
interface InstitutionControlsProps {
  institution: Institution;
  onChanged: () => void;
}
function InstitutionControls({
  institution,
  onChanged,
}: InstitutionControlsProps) {
  const [name, setName] = useState(institution.name);
  const [user, setUser] = useState("");
  const [reason, setReason] = useState("");
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const { refreshViewer } = useViewer();
  const run = async (operation: () => Promise<void>) => {
    if (busy) {
      return;
    }
    setBusy(true);
    setNotice(null);
    try {
      await operation();
      onChanged();
      await refreshViewer();
    } catch (reason) {
      setNotice(
        reason instanceof Error
          ? reason.message
          : "Не удалось изменить учреждение",
      );
    } finally {
      setBusy(false);
    }
  };
  const rename = (event: FormEvent) => {
    event.preventDefault();
    void run(() => renameInstitution(institution, name, reason));
  };
  const appoint = (event: FormEvent) => {
    event.preventDefault();
    const id = Number(user);
    if (!Number.isSafeInteger(id) || id <= 0) {
      setNotice("Укажите положительный ID аккаунта.");
      return;
    }
    void run(() =>
      setMembership(institution.id, id, {
        expectedVersion: 0,
        state: "active",
        roles: [],
        permissions: Object.keys(memberLabels),
        reason,
      }),
    );
  };
  return (
    <fieldset disabled={busy} className={styles.card}>
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
      <form className={styles.form} onSubmit={rename}>
        <label>
          Название
          <input
            required
            maxLength={200}
            value={name}
            onChange={(event) => setName(event.target.value)}
          />
        </label>
        <button disabled={reason.trim().length < 3} type="submit">
          Изменить название
        </button>
      </form>
      {institution.managerCount === 0 && (
        <form className={styles.form} onSubmit={appoint}>
          <label>
            ID первого управляющего
            <input
              required
              type="number"
              min="1"
              value={user}
              onChange={(event) => setUser(event.target.value)}
            />
          </label>
          <p>
            Если аккаунт уже состоит в учреждении, назначьте ему права через
            карточку участника ниже.
          </p>
          <button disabled={reason.trim().length < 3} type="submit">
            Назначить первого управляющего
          </button>
        </form>
      )}
      {notice && <p role="alert">{notice}</p>}
    </fieldset>
  );
}
