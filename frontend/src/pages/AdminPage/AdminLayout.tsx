import { useCallback } from "react";
import { Link, Navigate, Outlet, useLocation } from "react-router-dom";
import { getAdminAccess, type AdminAccess } from "../../api/admin";
import { useViewer } from "../../hooks/useViewer";
import { useApiResource } from "../../hooks/useApiResource";
import { AdminNavigation } from "../../components/admin/AdminNavigation";
import layout from "../../components/admin/AdminLayout.module.css";
import styles from "../../components/admin/Admin.module.css";

export function AdminLayout() {
  const { viewer, refreshViewer } = useViewer();
  const location = useLocation();
  const userId = viewer.kind === "authenticated" ? viewer.user.id : null;
  const signature =
    viewer.kind === "authenticated"
      ? JSON.stringify([viewer.user.permissions, viewer.user.memberships])
      : viewer.kind;
  const load = useCallback(
    (signal: AbortSignal) => {
      void signature;
      if (userId === null) {
        return Promise.resolve<AdminAccess | null>(null);
      }
      return getAdminAccess(signal);
    },
    [userId, signature],
  );
  const { result, loading, error, retry } = useApiResource(
    load,
    "Не удалось загрузить полномочия",
  );
  if (viewer.kind === "guest") {
    return (
      <Navigate
        to={`/login?redirect=${encodeURIComponent(location.pathname + location.search)}`}
        replace
      />
    );
  }
  if (viewer.kind === "error") {
    return (
      <main id="main-content" className={styles.page}>
        <p role="alert">{viewer.message}</p>
        <button type="button" onClick={() => void refreshViewer()}>
          Повторить
        </button>
      </main>
    );
  }
  if (viewer.kind === "loading" || loading) {
    return (
      <main id="main-content" className={styles.page}>
        <p role="status">Загружаем панель…</p>
      </main>
    );
  }
  if (error) {
    return (
      <main id="main-content" className={styles.page}>
        <p role="alert">{error}</p>
        <button type="button" onClick={retry}>
          Повторить
        </button>
      </main>
    );
  }
  if (
    !result ||
    (result.institutions.length === 0 &&
      !result.permissions.some((p) => p !== "view_account_private_data"))
  ) {
    return (
      <main id="main-content" className={styles.page}>
        <h1>Доступ не разрешён</h1>
        <Link to="/catalog">Вернуться в каталог</Link>
      </main>
    );
  }
  const platform = result.permissions.some(
    (permission) => permission !== "view_account_private_data",
  );
  const institutionMatch = location.pathname.match(
    /^\/admin\/institutions\/(\d+)/,
  );
  const scopedInstitution = result.institutions.find(
    (item) => item.id === Number(institutionMatch?.[1]),
  );
  let context = platform ? "Управление платформой" : "Управление учреждениями";
  if (scopedInstitution) {
    context = "Учреждение · " + scopedInstitution.name;
  } else if (institutionMatch) {
    context = "Управление учреждением";
  }
  return (
    <div key={userId} className={layout.layout}>
      <AdminNavigation access={result} platform={platform} />
      <main id="main-content" className={`${layout.content} ${styles.surface}`}>
        <div className={layout.context}>
          <span>{context}</span>
          <Link to="/catalog">Вернуться в каталог</Link>
        </div>
        <Outlet context={result} />
      </main>
    </div>
  );
}
