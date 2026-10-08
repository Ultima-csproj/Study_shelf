import { Link } from "react-router-dom";
import { useCallback } from "react";
import { useAdminAccess } from "../../hooks/useAdminAccess";
import { useApiResource } from "../../hooks/useApiResource";
import { getAdminOverview, platformLabels } from "../../api/admin";
import { AdminOverviewContent } from "../../components/admin/AdminOverviewContent";
import { AdminStatus } from "../../components/admin/AdminStatus";
import styles from "../../components/admin/Admin.module.css";
export function AdminHomePage() {
  const access = useAdminAccess();
  const load = useCallback(
    (signal: AbortSignal) => getAdminOverview(signal),
    [],
  );
  const overview = useApiResource(load, "Не удалось загрузить сводку");
  return (
    <section>
      <h1>Сводка</h1>
      <p>Показатели и ожидающие действия в доступных вам разделах.</p>
      {overview.loading && <p role="status">Загружаем сводку…</p>}
      {overview.error && (
        <AdminStatus
          message={overview.error}
          failure={overview.failure}
          retry={overview.retry}
        />
      )}
      {overview.result && <AdminOverviewContent overview={overview.result} />}
      <details className={styles.card}>
        <summary>Ваши возможности управления</summary>
        <ul>
          {access.permissions.map((p) => (
            <li key={p}>{platformLabels[p] ?? p}</li>
          ))}
        </ul>
      </details>
      <h2>Учреждения под вашим управлением</h2>
      {access.institutions.length === 0 && (
        <p>Управляемых учреждений пока нет.</p>
      )}
      <ul>
        {access.institutions.map((i) => (
          <li key={i.id}>
            <Link
              to={
                i.permissions.includes("manage_members")
                  ? `/admin/institutions/${i.id}`
                  : "/admin/materials/restricted"
              }
            >
              {i.name}
            </Link>
          </li>
        ))}
      </ul>
    </section>
  );
}
