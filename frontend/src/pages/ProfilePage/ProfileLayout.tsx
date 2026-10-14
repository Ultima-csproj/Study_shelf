import { useNavigationContext } from "../../hooks/useNavigationContext";
import type { ReactNode } from "react";
import { Link, useLocation } from "react-router-dom";
import { PencilSimpleIcon } from "@phosphor-icons/react/dist/csr/PencilSimple";
import type { User } from "../../types/viewer";
import { ProfileIdentityCard } from "./ProfileIdentityCard";
import styles from "./ProfilePage.module.css";
import { CaretLeftIcon } from "@phosphor-icons/react/dist/csr/CaretLeft";
import { ThemeToggle } from "../../components/common/ThemeToggle/ThemeToggle";

interface ProfileLayoutProps {
  user: User;
  children: ReactNode;
  editAction?: ReactNode;
  editing?: boolean;
  onCancel?: () => void;
}

export function ProfileLayout({
  user,
  children,
  editAction,
  editing = false,
  onCancel,
}: ProfileLayoutProps) {
  const { linkTo } = useNavigationContext("/profile");
  const location = useLocation();
  const publications =
    new URLSearchParams(location.search).get("tab") === "publications";
  const active = location.pathname.startsWith("/library")
    ? "saved"
    : location.pathname === "/settings"
      ? "settings"
      : publications
        ? "publications"
        : "about";
  const tabs = [
    { to: "/profile", label: "Обзор", key: "about" },
    {
      to: "/profile?tab=publications",
      label: "Публикации",
      key: "publications",
    },
    { to: "/library", label: "Библиотека", key: "saved" },
    { to: "/settings", label: "Настройки", key: "settings" },
  ];
  const edit =
    editAction === undefined ? (
      <Link to={linkTo("/profile?edit=1")} className={styles.secondary}>
        <PencilSimpleIcon size={20} aria-hidden="true" />
        Редактировать профиль
      </Link>
    ) : (
      editAction
    );

  return (
    <div
      className={`${styles.page} ${editing ? styles.editing : ""}`}
      data-profile-editing={editing}
      data-profile-section={active}
    >
      {active === "about" && !editing && (
        <header className={styles.mobileIntro}>
          <h1>Мой профиль</h1>
          <p>Личная информация и настройки</p>
        </header>
      )}
      {editing && (
        <header className={styles.mobileEditorHeader}>
          <button
            type="button"
            onClick={onCancel}
            aria-label="Отменить редактирование"
          >
            <CaretLeftIcon size={24} aria-hidden="true" />
          </button>
          <h1>Редактирование профиля</h1>
          <ThemeToggle />
        </header>
      )}
      <ProfileIdentityCard user={user} editAction={edit} />
      <div
        className={`${styles.profileBody} ${active === "about" && !editing ? styles.overviewBody : ""}`}
      >
        <nav className={styles.profileTabs} aria-label="Разделы профиля">
          {tabs.map((tab) => (
            <Link
              key={tab.key}
              to={tab.to}
              aria-current={active === tab.key ? "page" : undefined}
            >
              {tab.label}
            </Link>
          ))}
        </nav>
        {typeof location.state?.notice === "string" && (
          <p className={styles.notice} role="status">
            {location.state.notice}
          </p>
        )}
        {children}
      </div>
    </div>
  );
}
