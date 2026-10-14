import { useState } from "react";
import {
  changeMaterialState,
  restrictMaterial,
  unrestrictMaterial,
  type Workspace,
} from "../../../api/materialWorkspace";
import styles from "../shared/Panel.module.css";

interface MaterialStateControlsProps {
  workspace: Workspace;
  busy: boolean;
  reason: string;
  run: (operation: () => Promise<unknown>) => Promise<boolean>;
}
export function MaterialStateControls({
  workspace,
  busy,
  reason,
  run,
}: MaterialStateControlsProps) {
  const material = workspace.material;
  const capabilities = workspace.capabilities;
  const scopedInstitutions = capabilities.restrictInstitutions.filter(
    (id) =>
      id === material.ownerInstitutionId ||
      workspace.placements.some((placement) => placement.institutionId === id),
  );
  const [scope, setScope] = useState(
    capabilities.restrictGlobal ? "" : String(scopedInstitutions[0] ?? ""),
  );
  const ready = !busy && reason.trim().length >= 3;
  const pending = material.workingRevisionState === "pending_moderation";
  const canRestrict =
    capabilities.restrictGlobal || scopedInstitutions.length > 0;
  return (
    <section data-material-panel className={styles.card}>
      <h2>Доступность материала</h2>
      <p>
        {material.visibility === "private" ? "Приватный" : "Публичный"} ·{" "}
        {material.publication === "published"
          ? "Опубликован"
          : "Снят с публикации"}
        {material.lifecycle === "archived" && " · В архиве"}
      </p>
      {capabilities.manage && (
        <div className={styles.actions}>
          <button
            type="button"
            disabled={!ready || pending}
            onClick={() =>
              void run(() =>
                changeMaterialState(
                  material,
                  material.lifecycle === "archived" ? "restore" : "archive",
                  reason,
                ),
              )
            }
          >
            {material.lifecycle === "archived"
              ? "Восстановить из архива"
              : "В архив"}
          </button>
          {material.publishedRevisionId !== null && (
            <button
              type="button"
              disabled={
                !ready ||
                pending ||
                (material.publication === "unpublished" &&
                  material.visibility === "private")
              }
              onClick={() =>
                void run(() =>
                  changeMaterialState(
                    material,
                    material.publication === "published"
                      ? "unpublish"
                      : "republish",
                    reason,
                  ),
                )
              }
            >
              {material.publication === "published"
                ? "Снять с публикации"
                : "Вернуть прежнюю публикацию"}
            </button>
          )}
          {material.visibility === "public" && (
            <button
              type="button"
              disabled={!ready || pending}
              onClick={() =>
                void run(() => changeMaterialState(material, "private", reason))
              }
            >
              Сделать приватным
            </button>
          )}
        </div>
      )}
      {pending && (
        <p>
          Перед изменением доступности отзовите рабочую редакцию из очереди
          модерации.
        </p>
      )}
      {material.visibility === "private" && (
        <p>
          Публичный доступ появляется после принятия отправленной редакции
          модератором.
        </p>
      )}
      <h3>Административные ограничения</h3>
      {workspace.restrictions.length === 0 && (
        <p>Действующих ограничений нет.</p>
      )}
      {workspace.restrictions.map((restriction) => {
        const allowed =
          restriction.institutionId === null
            ? capabilities.restrictGlobal
            : capabilities.restrictInstitutions.includes(
                restriction.institutionId,
              );
        return (
          <div key={restriction.id}>
            <p>
              {restriction.institutionName ?? "Вся платформа"}:{" "}
              {restriction.reason}
            </p>
            {allowed && (
              <div className={styles.actions}>
                <button
                  type="button"
                  disabled={!ready}
                  onClick={() =>
                    void run(() =>
                      unrestrictMaterial(material, restriction.id, reason),
                    )
                  }
                >
                  Снять это ограничение
                </button>
              </div>
            )}
          </div>
        );
      })}
      {canRestrict &&
        material.visibility === "public" &&
        material.publishedRevisionId !== null && (
          <div className={styles.actions}>
            <label className={styles.label}>
              Область ограничения
              <select
                value={scope}
                onChange={(event) => setScope(event.target.value)}
              >
                {capabilities.restrictGlobal && (
                  <option value="">Вся платформа</option>
                )}
                {scopedInstitutions.map((id) => (
                  <option key={id} value={id}>
                    Учреждение #{id}
                  </option>
                ))}
              </select>
            </label>
            <button
              type="button"
              disabled={!ready}
              onClick={() =>
                void run(() =>
                  restrictMaterial(
                    material,
                    scope === "" ? null : Number(scope),
                    reason,
                  ),
                )
              }
            >
              Ограничить доступ
            </button>
          </div>
        )}
      {workspace.restrictions.length > 0 && (
        <p className={styles.banner}>
          Восстановление из архива и повторная публикация не снимают
          административные ограничения.
        </p>
      )}
    </section>
  );
}
