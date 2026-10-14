import { useCallback } from "react";
import { FilesIcon } from "@phosphor-icons/react/dist/csr/Files";
import { FileTextIcon } from "@phosphor-icons/react/dist/csr/FileText";
import { ClockIcon } from "@phosphor-icons/react/dist/csr/Clock";
import { getAuthorMaterials } from "../../../api/materialWorkspace";
import { useApiResource } from "../../../hooks/useApiResource";
import { Button } from "../../common/Button/Button";
import styles from "./AuthorStats.module.css";

export function AuthorStats({ userId }: { userId: number }) {
  const load = useCallback(
    async (signal: AbortSignal) => {
      void userId;
      const counts = await Promise.all(
        ["active", "archived", "draft", "pending_moderation"].map(
          async (state) => {
            const result = await getAuthorMaterials(
              { page: 1, q: "", state, order: "newest" },
              signal,
            );
            return result.total;
          },
        ),
      );
      return {
        total: counts[0] + counts[1],
        draft: counts[2],
        pending: counts[3],
      };
    },
    [userId],
  );
  const { result, loading, error, retry } = useApiResource(
    load,
    "Не удалось загрузить счётчики",
  );
  const items = [
    {
      label: "Всего материалов",
      compactLabel: "Всего",
      value: result?.total,
      Icon: FilesIcon,
    },
    {
      label: "Черновики",
      compactLabel: "Черновики",
      value: result?.draft,
      Icon: FileTextIcon,
    },
    {
      label: "На проверке",
      compactLabel: "На проверке",
      value: result?.pending,
      Icon: ClockIcon,
    },
  ];
  return (
    <section aria-label="Статистика публикаций" aria-busy={loading}>
      <dl className={styles.stats}>
        {items.map(({ label, compactLabel, value, Icon }) => (
          <div key={label} className={styles.stat}>
            <span className={styles.statIcon}>
              <Icon size={30} aria-hidden="true" />
            </span>
            <div>
              <dt>
                <span className={styles.desktopIntro}>{label}</span>
                <span className={styles.mobileIntro}>{compactLabel}</span>
              </dt>
              <dd>{value ?? "—"}</dd>
            </div>
          </div>
        ))}
      </dl>
      {error && (
        <p className={styles.statsError} role="status">
          {error} <Button onClick={retry}>Повторить</Button>
        </p>
      )}
    </section>
  );
}
