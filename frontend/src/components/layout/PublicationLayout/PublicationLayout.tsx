import type { ReactNode } from "react";
import { Link, NavLink } from "react-router-dom";
import { UserIcon } from "@phosphor-icons/react/dist/csr/User";
import { BookOpenIcon } from "@phosphor-icons/react/dist/csr/BookOpen";
import { FileTextIcon } from "@phosphor-icons/react/dist/csr/FileText";
import { GearIcon } from "@phosphor-icons/react/dist/csr/Gear";
import { ThemeToggle } from "../../common/ThemeToggle/ThemeToggle";
import styles from "./PublicationLayout.module.css";

const links = [
  { to: "/profile", label: "Профиль", mobileLabel: "Профиль", Icon: UserIcon },
  { to: "/library", label: "Моя библиотека", mobileLabel: "Библиотека", Icon: BookOpenIcon },
  { to: "/author/materials", label: "Мои публикации", mobileLabel: "Публикации", Icon: FileTextIcon },
  { to: "/settings", label: "Настройки", mobileLabel: "Настройки", Icon: GearIcon },
];

export function PublicationLayout({ children }: { children: ReactNode }) {
  return (
    <div className={styles.layout}>
      <a href="#main-content" className="skip-link">Перейти к содержимому</a>
      <header className={styles.mobileHeader}>
        <Link to="/catalog" className={styles.brand}>
          <BookOpenIcon size={30} aria-hidden="true" />
          <span>Учебная полка</span>
        </Link>
        <ThemeToggle />
      </header>
      <aside className={styles.sidebar}>
        <Link to="/catalog" className={styles.brand} aria-label="Учебная полка — каталог">
          <BookOpenIcon size={30} weight="fill" aria-hidden="true" />
          <span>Учебная полка</span>
        </Link>
        <nav aria-label="Разделы личной зоны">
          {links.map(({ to, label, mobileLabel, Icon }) => (
            <NavLink
              key={to}
              to={to}
              aria-label={label}
              className={({ isActive }) => isActive ? styles.selected : ""}
            >
              <Icon size={24} aria-hidden="true" />
              <span className={styles.desktopLabel}>{label}</span>
              <span className={styles.mobileLabel}>{mobileLabel}</span>
            </NavLink>
          ))}
        </nav>
        <div className={styles.theme}><ThemeToggle /></div>
      </aside>
      <main id="main-content" className={styles.content} tabIndex={-1}>
        {children}
      </main>
    </div>
  );
}
