import { Navigate, NavLink, Outlet, useLocation } from "react-router-dom";
import { UserIcon } from "@phosphor-icons/react/dist/csr/User";
import { BookOpenIcon } from "@phosphor-icons/react/dist/csr/BookOpen";
import { GearIcon } from "@phosphor-icons/react/dist/csr/Gear";
import { useViewer } from "../../../hooks/useViewer";
import styles from "./PersonalLayout.module.css";

const links = [
  { to: "/profile", label: "Профиль", Icon: UserIcon },
  { to: "/library", label: "Моя библиотека", Icon: BookOpenIcon },
  { to: "/author/materials", label: "Мои публикации", Icon: BookOpenIcon },
  { to: "/settings", label: "Настройки", Icon: GearIcon },
];

export function PersonalLayout() {
  const { viewer, refreshViewer } = useViewer();
  const location = useLocation();

  if (viewer.kind === "loading") {
    return (
      <main id="main-content">
        <p role="status">Загружаем аккаунт…</p>
      </main>
    );
  }
  if (viewer.kind === "error") {
    return (
      <main id="main-content">
        <p role="alert">{viewer.message}</p>
        <button type="button" onClick={() => void refreshViewer()}>
          Повторить
        </button>
      </main>
    );
  }

  if (viewer.kind === "guest") {
    const redirect = location.pathname + location.search + location.hash;
    const login = `/login?redirect=${encodeURIComponent(redirect)}`;

    return <Navigate to={login} replace />;
  }

  return (
    <div key={viewer.user.id} className={styles.layout}>
      <aside className={styles.sidebar}>
        <h2>Личная зона</h2>
        <nav aria-label="Разделы личной зоны">
          {links.map(({ to, label, Icon }) => (
            <NavLink
              key={to}
              to={to}
              className={({ isActive }) => getNavigationLinkClassName(isActive)}
            >
              <Icon size={25} aria-hidden="true" />
              {label}
            </NavLink>
          ))}
        </nav>
      </aside>
      <main id="main-content" className={styles.content} tabIndex={-1}>
        <Outlet />
      </main>
    </div>
  );
}

function getNavigationLinkClassName(isActive: boolean): string {
  if (isActive) {
    return styles.selected;
  }

  return "";
}
