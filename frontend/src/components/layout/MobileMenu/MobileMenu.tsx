import type { MouseEvent, ReactNode, RefObject } from "react";
import { Link, NavLink, useLocation } from "react-router-dom";
import { XIcon } from "@phosphor-icons/react/dist/csr/X";
import { useViewer } from "../../../hooks/useViewer";
import { roleLabels } from "../../../utils/viewerLabels";
import { loginPath } from "../../../utils/redirect";
import { materialNavigationSection } from "../../../utils/navigation";
import styles from "./MobileMenu.module.css";

interface MobileMenuProps {
  dialogRef: RefObject<HTMLDialogElement | null>;
  onClose: () => void;
}

export function MobileMenu({
  dialogRef,
  onClose,
}: MobileMenuProps) {
  const { viewer, signOut } = useViewer();
  const location = useLocation();
  const login = loginPath(location);
  const materialSection = materialNavigationSection(location);
  const selectedSection = materialSection ?? location.pathname.split("/")[1];
  const close = () => dialogRef.current?.close();
  let catalogCurrent: "location" | undefined = undefined;
  let disciplinesCurrent: "location" | undefined = undefined;
  let collectionsCurrent: "location" | undefined = undefined;

  if (selectedSection === "catalog") {
    catalogCurrent = "location";
  }

  if (selectedSection === "disciplines") {
    disciplinesCurrent = "location";
  }

  if (selectedSection === "collections") {
    collectionsCurrent = "location";
  }

  const handleBackdropClick = (event: MouseEvent<HTMLDialogElement>) => {
    const clickedBackdrop = event.target === event.currentTarget;

    if (clickedBackdrop) {
      close();
    }
  };

  const handleSignOut = () => {
    signOut();
    close();
  };

  let accountNavigation: ReactNode;

  if (viewer.kind === "guest") {
    accountNavigation = (
      <>
        <NavLink to={login} onClick={close}>
          Войти
        </NavLink>
        <NavLink to={`${login}&mode=register`} onClick={close}>
          Создать аккаунт
        </NavLink>
      </>
    );
  } else {
    accountNavigation = (
      <>
        <p className={styles.user}>
          {viewer.user.name} · {roleLabels[viewer.user.role]}
        </p>
        <NavLink to="/profile" onClick={close}>
          Профиль
        </NavLink>
        <NavLink to="/library" onClick={close}>
          Моя библиотека
        </NavLink>
        <NavLink to="/settings" onClick={close}>
          Настройки
        </NavLink>
        <button
          type="button"
          onClick={handleSignOut}
        >
          Выйти
        </button>
      </>
    );
  }

  return (
    <dialog
      ref={dialogRef}
      className={styles.dialog}
      aria-label="Меню сайта"
      onClose={onClose}
      onClick={handleBackdropClick}
    >
      <div className={styles.heading}>
        <strong>Меню</strong>
        <button type="button" aria-label="Закрыть меню" onClick={close}>
          <XIcon size={22} aria-hidden="true" />
        </button>
      </div>
      <nav aria-label="Мобильная навигация">
        <Link
          to="/catalog"
          aria-current={catalogCurrent}
          onClick={close}
        >
          Каталог
        </Link>
        <Link
          to="/disciplines"
          aria-current={disciplinesCurrent}
          onClick={close}
        >
          Дисциплины
        </Link>
        <Link
          to="/collections"
          aria-current={collectionsCurrent}
          onClick={close}
        >
          Подборки
        </Link>
        <hr />
        {accountNavigation}
      </nav>
    </dialog>
  );
}
