import { useCallback, useState } from "react";
import { Link } from "react-router-dom";
import { AdminList, type RunAction } from "../../components/admin/AdminList";
import {
  getRestrictedMaterials,
  getRestrictionMetadata,
  unrestrictMaterial,
  type MaterialHeader,
} from "../../api/materialWorkspace";
import { useApiResource } from "../../hooks/useApiResource";
import { useAdminAccess } from "../../hooks/useAdminAccess";
import styles from "../../components/materials/shared/Panel.module.css";

export function AdminRestrictedMaterialsPage() {
  return (
    <AdminList
      title="Ограничения материалов"
      load={getRestrictedMaterials}
      emptyMessage="Действующих ограничений пока нет"
      render={(material, run) => (
        <RestrictedCard
          key={material.id + ":" + material.version}
          material={material}
          run={run}
        />
      )}
    />
  );
}
interface RestrictedCardProps {
  material: MaterialHeader;
  run: RunAction;
}
function RestrictedCard({ material, run }: RestrictedCardProps) {
  const access = useAdminAccess();
  const [reason, setReason] = useState("");
  const load = useCallback(
    (signal: AbortSignal) => getRestrictionMetadata(material.id, signal),
    [material.id],
  );
  const { result, loading, error, retry } = useApiResource(
    load,
    "Не удалось прочитать ограничения",
  );
  const global = access.permissions.includes("restrict_public_materials");
  const globalRead = access.permissions.includes("review_restricted_materials");
  return (
    <article className={styles.card}>
      <h2>{material.title}</h2>
      <p>
        Материал #{material.id} ·{" "}
        {material.lifecycle === "archived" ? "В архиве" : "Активен"}
      </p>
      {material.visibility === "private" && (
        <p>
          Приватное содержимое доступно владельцу и приглашённым. Здесь показаны
          только сведения об ограничении.
        </p>
      )}
      {loading && <p role="status">Загрузка ограничений…</p>}
      {error && (
        <p role="alert">
          {error}{" "}
          <button type="button" onClick={retry}>
            Повторить
          </button>
        </p>
      )}
      <label className={styles.label}>
        Причина снятия ограничения
        <textarea
          minLength={3}
          maxLength={2000}
          value={reason}
          onChange={(event) => setReason(event.target.value)}
        />
      </label>
      {result?.map((restriction) => {
        const scope = access.institutions.find(
          (institution) => institution.id === restriction.institutionId,
        );
        const canChange =
          restriction.institutionId === null
            ? global
            : scope?.permissions.includes("restrict_materials");
        const canRead =
          globalRead ||
          scope?.permissions.includes("review_restricted_materials");
        return (
          <div key={restriction.id}>
            <p>
              {restriction.institutionName ?? "Вся платформа"}:{" "}
              {restriction.reason}
            </p>
            <div className={styles.actions}>
              {material.visibility === "public" && canRead && (
                <Link
                  to={
                    "/materials/" +
                    material.id +
                    "?restrictionId=" +
                    restriction.id
                  }
                >
                  Просмотреть последнюю публикацию с отметкой ограничения
                </Link>
              )}
              {canChange && (
                <button
                  type="button"
                  disabled={reason.trim().length < 3}
                  onClick={() =>
                    void run(() =>
                      unrestrictMaterial(material, restriction.id, reason),
                    )
                  }
                >
                  Снять это ограничение
                </button>
              )}
            </div>
          </div>
        );
      })}
    </article>
  );
}
