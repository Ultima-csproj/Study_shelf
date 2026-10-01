import type { RefObject } from "react";
import { Link, NavLink, useLocation } from "react-router-dom";
import { XIcon } from "@phosphor-icons/react/dist/csr/X";
import { useViewer } from "../../../hooks/useViewer";
import { roleLabels } from "../../../utils/viewerLabels";
import { loginPath } from "../../../utils/redirect";
import { materialNavigationSection } from "../../../utils/navigation";
import styles from "./MobileMenu.module.css";
export function MobileMenu({
  dialogRef,
  onClose,
}: {
  dialogRef: RefObject<HTMLDialogElement | null>;
  onClose: () => void;
}) {
  const { viewer, signOut } = useViewer();
  const location = useLocation();
  const login = loginPath(location);
  const materialSection = materialNavigationSection(location);
  const selectedSection = materialSection ?? location.pathname.split("/")[1];
  const close = () => dialogRef.current?.close();
  return (
    <dialog
      ref={dialogRef}
      className={styles.dialog}
      aria-label="Меню сайта"
      onClose={onClose}
      onClick={(event) => {
        if (event.target === event.currentTarget) {
          close();
        }
      }}
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
          aria-current={selectedSection === "catalog" ? "location" : undefined}
          onClick={close}
        >
          Каталог
        </Link>
        <Link
          to="/disciplines"
          aria-current={
            selectedSection === "disciplines" ? "location" : undefined
          }
          onClick={close}
        >
          Дисциплины
        </Link>
        <Link
          to="/collections"
          aria-current={
            selectedSection === "collections" ? "location" : undefined
          }
          onClick={close}
        >
          Подборки
        </Link>
        <hr />
        {viewer.kind === "guest" ? (
          <>
            <NavLink to={login} onClick={close}>
              Войти
            </NavLink>
            <NavLink to={`${login}&mode=register`} onClick={close}>
              Создать аккаунт
            </NavLink>
          </>
        ) : (
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
              onClick={() => {
                signOut();
                close();
              }}
            >
              Выйти
            </button>
          </>
        )}
      </nav>
    </dialog>
  );
}
