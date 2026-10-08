import { useCallback, useState, type FormEvent } from "react";
import {
  getOwnInstitutionRequests,
  registerInstitution,
  stateLabels,
} from "../../api/admin";
import { useApiResource } from "../../hooks/useApiResource";
import styles from "./AccountForm.module.css";
export function InstitutionRegistration() {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [evidence, setEvidence] = useState("");
  const [page, setPage] = useState(1);
  const [revision, setRevision] = useState(0);
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const load = useCallback(
    (signal: AbortSignal) => {
      void revision;
      return getOwnInstitutionRequests({ page, q: "", state: "" }, signal);
    },
    [page, revision],
  );
  const { result, loading, error, retry } = useApiResource(
    load,
    "Не удалось загрузить заявки учреждений",
  );
  const submit = async (event: FormEvent) => {
    event.preventDefault();
    if (busy) {
      return;
    }
    setBusy(true);
    setNotice(null);
    try {
      await registerInstitution(name, description, evidence);
      setRevision((r) => r + 1);
      setNotice(
        "Заявка отправлена. После подтверждения вы станете первым управляющим учреждения.",
      );
    } catch (reason) {
      setNotice(
        reason instanceof Error
          ? reason.message
          : "Не удалось отправить заявку",
      );
    } finally {
      setBusy(false);
    }
  };
  return (
    <section className={styles.section}>
      <h2>Зарегистрировать учреждение</h2>
      <p>
        Платформа рассмотрит заявку и основание представительства. До
        подтверждения учреждение не появляется в справочнике.
      </p>
      <form className={styles.form} onSubmit={submit}>
        <fieldset disabled={busy}>
          <label>
            Название
            <input
              required
              maxLength={200}
              value={name}
              onChange={(event) => setName(event.target.value)}
            />
          </label>
          <label>
            Описание
            <textarea
              maxLength={2000}
              value={description}
              onChange={(event) => setDescription(event.target.value)}
            />
          </label>
          <label>
            Основание представительства
            <textarea
              required
              minLength={3}
              maxLength={2000}
              value={evidence}
              onChange={(event) => setEvidence(event.target.value)}
            />
          </label>
          <button type="submit">Отправить заявку</button>
        </fieldset>
      </form>
      {notice && <p role="status">{notice}</p>}
      {loading && <p role="status">Загружаем…</p>}
      {error && (
        <p role="alert">
          {error}{" "}
          <button type="button" onClick={retry}>
            Повторить
          </button>
        </p>
      )}
      <ul>
        {result?.items.map((r) => (
          <li key={r.id}>
            {r.name} — {stateLabels[r.state]}
            {r.institutionId !== null && ` · Учреждение #${r.institutionId}`}
            {r.decisionReason && <p>Причина решения: {r.decisionReason}</p>}
          </li>
        ))}
      </ul>
      {result && (
        <div className={styles.actions}>
          <button
            type="button"
            disabled={page <= 1}
            onClick={() => setPage((p) => p - 1)}
          >
            Назад
          </button>
          <span>Страница {page}</span>
          <button
            type="button"
            disabled={page >= result.pageCount}
            onClick={() => setPage((p) => p + 1)}
          >
            Далее
          </button>
        </div>
      )}
    </section>
  );
}
