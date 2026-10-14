import { useCallback, useState, type FormEvent } from "react";
import {
  getIdentityRequests,
  requestContributor,
  requestMembership,
} from "../../api/profile";
import { getJson } from "../../api/http";
import { useApiResource } from "../../hooks/useApiResource";
import styles from "../common/Form/Form.module.css";
import { useViewer } from "../../hooks/useViewer";
import { StatusBadge } from "../common/StatusBadge/StatusBadge";
import requestStyles from "./IdentityRequests.module.css";
import { Button } from "../common/Button/Button";

interface Institution {
  id: number;
  name: string;
}
export function IdentityRequests() {
  const { viewer } = useViewer();
  const [version, setVersion] = useState(0);
  const [institution, setInstitution] = useState("");
  const [contributor, setContributor] = useState("");
  const [evidence, setEvidence] = useState("");
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const load = useCallback(
    async (signal: AbortSignal) => {
      void version;
      const [requests, institutions] = await Promise.all([
        getIdentityRequests(signal),
        getJson<{ items: Institution[] }>(
          "/api/institutions?pageSize=100",
          signal,
        ),
      ]);
      return { requests, institutions: institutions.items };
    },
    [version],
  );
  const { result, loading, error, retry } = useApiResource(
    load,
    "Не удалось загрузить заявки",
  );
  const existingMembership = result?.requests.memberships.find(
    (item) => item.id === Number(institution),
  );
  const cannotApply =
    existingMembership?.state === "active" ||
    existingMembership?.state === "pending";

  const submit = async (
    event: FormEvent,
    kind: "membership" | "contributor",
  ) => {
    event.preventDefault();
    if (busy) {
      return;
    }
    const id = Number(kind === "membership" ? institution : contributor);
    if (!Number.isSafeInteger(id) || id <= 0) {
      setNotice(
        "Выберите учреждение или укажите положительный ID автора из карточки материала.",
      );
      return;
    }
    setBusy(true);
    setNotice(null);
    try {
      if (kind === "membership") {
        await requestMembership(id);
      } else {
        await requestContributor(id, evidence);
      }
      setNotice("Заявка отправлена на подтверждение.");
      setVersion((value) => value + 1);
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
    <section id="institution-membership" className={styles.section}>
      <h2>Учреждение и авторство</h2>
      <p>
        Подтверждённые роли назначаются после рассмотрения заявки. Привязка
        автора не меняет права доступа к материалам.
      </p>
      {loading && <p role="status">Загружаем заявки…</p>}
      {error && (
        <p role="alert">
          {error}{" "}
          <button type="button" onClick={retry}>
            Повторить
          </button>
        </p>
      )}
      {notice && <p role="status">{notice}</p>}
      <form
        className={styles.form}
        onSubmit={(event) => void submit(event, "membership")}
      >
        <fieldset disabled={busy}>
          <p>
            Это заявка от вашего аккаунта
            {viewer.kind === "authenticated" ? ` «${viewer.user.name}»` : ""}.
            Она не меняет членство других пользователей.
          </p>
          <label htmlFor="membership-institution">Учреждение</label>
          <select
            id="membership-institution"
            required
            value={institution}
            onChange={(event) => setInstitution(event.target.value)}
          >
            <option value="">Выберите учреждение</option>
            {result?.institutions.map((item) => (
              <option key={item.id} value={item.id}>
                {item.name}
              </option>
            ))}
          </select>
          <Button
            variant="primary"
            type="submit"
            loading={busy}
            disabled={loading || !result || cannotApply}
          >
            Подать свою заявку на вступление
          </Button>
          {cannotApply && (
            <p>
              {existingMembership.state === "active"
                ? "Вы уже состоите в этом учреждении."
                : "Ваша заявка уже ожидает рассмотрения."}
            </p>
          )}
        </fieldset>
      </form>
      <form
        className={styles.form}
        onSubmit={(event) => void submit(event, "contributor")}
      >
        <fieldset disabled={busy}>
          <label htmlFor="contributor-link">ID существующего автора</label>
          <input
            id="contributor-link"
            type="number"
            min="1"
            required
            value={contributor}
            onChange={(event) => setContributor(event.target.value)}
          />
          <label htmlFor="contributor-evidence">Основание привязки</label>
          <textarea
            id="contributor-evidence"
            required
            minLength={3}
            maxLength={2000}
            value={evidence}
            onChange={(event) => setEvidence(event.target.value)}
          />
          <Button variant="primary" type="submit" loading={busy}>
            Запросить привязку автора
          </Button>
        </fieldset>
      </form>
      <ul className={requestStyles.requests}>
        {result?.requests.memberships.map((r) => (
          <li key={`institution:${r.id}`}>
            {r.name} <StatusBadge state={r.state} />
          </li>
        ))}
        {result?.requests.contributors.map((r) => (
          <li key={`contributor:${r.id}`}>
            {r.name} — {r.state}
          </li>
        ))}
      </ul>
    </section>
  );
}
