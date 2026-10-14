import { useState } from "react";
import { Link, useSearchParams } from "react-router-dom";
import {
  getContributorRequests,
  getInstitutionRequests,
  decideRequest,
  stateLabels,
  type AdminRequest,
} from "../../api/admin";
import { AdminList, type RunAction } from "../../components/admin/AdminList";
import { useAdminAccess } from "../../hooks/useAdminAccess";
import styles from "../../components/admin/Admin.module.css";
import { StatusBadge } from "../../components/common/StatusBadge/StatusBadge";

export function AdminRequestsPage() {
  const access = useAdminAccess();
  const [params] = useSearchParams();
  const account = access.permissions.includes("manage_accounts");
  const institution = access.permissions.includes("manage_institutions");
  const kind =
    params.get("kind") === "institutions" || !account
      ? "institutions"
      : "contributors";
  if (
    (kind === "institutions" && !institution) ||
    (kind === "contributors" && !account)
  ) {
    return <p>Нет доступа к этому виду заявок.</p>;
  }
  const load =
    kind === "contributors" ? getContributorRequests : getInstitutionRequests;
  const privateData = access.permissions.includes("view_account_private_data");
  return (
    <section>
      <nav className={styles.actions}>
        {account && (
          <Link to="/admin/requests?kind=contributors">Привязка авторов</Link>
        )}
        {institution && (
          <Link to="/admin/requests?kind=institutions">
            Регистрация учреждений
          </Link>
        )}
      </nav>
      <p>
        Заявки на членство находятся в карточке соответствующего учреждения.
      </p>
      <AdminList
        key={kind}
        title={
          kind === "contributors"
            ? "Привязки авторов"
            : "Регистрация учреждений"
        }
        load={load}
        states={["pending", "accepted", "rejected"]}
        stateLabels={stateLabels}
        render={(r, run) => (
          <RequestDecision
            key={`${r.id}:${r.userId}:${r.version}`}
            request={r}
            kind={kind}
            privateData={privateData}
            run={run}
          />
        )}
      />
    </section>
  );
}
interface RequestDecisionProps {
  request: AdminRequest;
  kind: "contributors" | "institutions";
  privateData: boolean;
  run: RunAction;
}
function RequestDecision({
  request,
  kind,
  privateData,
  run,
}: RequestDecisionProps) {
  const [reason, setReason] = useState("");
  const ready = reason.trim().length >= 3;
  return (
    <article className={styles.card}>
      <h2>{request.name}</h2>
      <p>
        Заявитель: {request.userName} · #{request.userId}
      </p>
      <p>
        <StatusBadge state={request.state} label={stateLabels[request.state]} />{" "}
        · {request.createdAt}
      </p>
      <p>{request.description}</p>
      {request.decisionReason && (
        <p>Причина решения: {request.decisionReason}</p>
      )}
      {request.evidence !== null ? (
        <p>Основание: {request.evidence || "Не указано"}</p>
      ) : (
        <p>
          Основание доступно сотрудникам с правом просмотра персональных данных.
        </p>
      )}
      {request.state === "pending" && (
        <div className={styles.form}>
          <label>
            Причина решения
            <textarea
              required
              minLength={3}
              maxLength={2000}
              value={reason}
              onChange={(event) => setReason(event.target.value)}
            />
          </label>
          <div className={styles.actions}>
            <button
              type="button"
              disabled={
                !ready ||
                (kind === "contributors" &&
                  (!privateData || !request.evidence?.trim()))
              }
              onClick={() =>
                void run(() => decideRequest(kind, request, true, reason))
              }
            >
              Подтвердить
            </button>
            <button
              type="button"
              disabled={!ready}
              onClick={() =>
                void run(() => decideRequest(kind, request, false, reason))
              }
            >
              Отклонить
            </button>
          </div>
        </div>
      )}
    </article>
  );
}
