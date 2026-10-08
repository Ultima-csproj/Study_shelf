import { useCallback, useState } from "react";
import { Link } from "react-router-dom";
import {
  getRestrictionTarget,
  restrictMaterial,
} from "../../api/materialWorkspace";
import { useApiResource } from "../../hooks/useApiResource";
import { useMaterialAction } from "../../hooks/useMaterialAction";
import { useViewer } from "../../hooks/useViewer";
import styles from "./Workspace.module.css";

interface PublicMaterialRestrictionPanelProps {
  id: number;
}

function RestrictionForm({ id }: PublicMaterialRestrictionPanelProps) {
  const [counter, setCounter] = useState(0);
  const [reason, setReason] = useState("");
  const [scope, setScope] = useState<string | null>(null);
  const load = useCallback(
    (signal: AbortSignal) => {
      void counter;
      return getRestrictionTarget(id, signal);
    },
    [id, counter],
  );
  const { result, loading, error, retry } = useApiResource(
    load,
    "Недоступно управление ограничением",
  );
  const changed = useCallback(() => setCounter((value) => value + 1), []);
  const action = useMaterialAction(changed);
  const selected =
    scope ?? (result?.global ? "" : String(result?.institutions[0] ?? ""));
  return (
    <section className={styles.card}>
      <h2>Ограничить публичный материал</h2>
      {loading && <p role="status">Загрузка полномочий…</p>}
      {error && (
        <p role="alert">
          {error}{" "}
          <button type="button" onClick={retry}>
            Повторить
          </button>
        </p>
      )}
      {action.notice && (
        <p role="status">
          {action.notice}{" "}
          <button type="button" onClick={changed}>
            Обновить данные
          </button>
        </p>
      )}
      {result && (
        <>
          <label className={styles.label}>
            Область ограничения
            <select
              value={selected}
              onChange={(event) => setScope(event.target.value)}
            >
              {result.global && <option value="">Вся платформа</option>}
              {result.institutions.map((institution) => (
                <option key={institution} value={institution}>
                  Учреждение #{institution}
                </option>
              ))}
            </select>
          </label>
          <label className={styles.label}>
            Причина
            <textarea
              minLength={3}
              maxLength={2000}
              value={reason}
              onChange={(event) => setReason(event.target.value)}
            />
          </label>
          <div className={styles.actions}>
            <button
              type="button"
              disabled={action.busy || reason.trim().length < 3}
              onClick={() =>
                void action.run(() =>
                  restrictMaterial(
                    result.material,
                    selected === "" ? null : Number(selected),
                    reason,
                  ),
                )
              }
            >
              Ограничить доступ
            </button>
            <Link to="/admin/materials/restricted">
              Действующие ограничения и их снятие
            </Link>
          </div>
          <p>
            Учебные связи и история сохранятся. Чтобы увидеть изменившуюся
            доступность, обновите страницу материала.
          </p>
        </>
      )}
    </section>
  );
}

export function PublicMaterialRestrictionPanel({
  id,
}: PublicMaterialRestrictionPanelProps) {
  const { viewer } = useViewer();
  if (viewer.kind !== "authenticated") {
    return null;
  }
  const allowed =
    viewer.user.permissions.includes("restrict_public_materials") ||
    viewer.user.memberships.some(
      (membership) =>
        membership.state === "active" &&
        membership.permissions.includes("restrict_materials"),
    );
  if (!allowed) {
    return null;
  }
  return <RestrictionForm key={viewer.user.id + ":" + id} id={id} />;
}
