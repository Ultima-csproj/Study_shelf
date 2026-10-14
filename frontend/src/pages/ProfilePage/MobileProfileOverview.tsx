import { useCallback, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { BookOpenIcon } from "@phosphor-icons/react/dist/csr/BookOpen";
import { FileTextIcon } from "@phosphor-icons/react/dist/csr/FileText";
import { GraduationCapIcon } from "@phosphor-icons/react/dist/csr/GraduationCap";
import { GearIcon } from "@phosphor-icons/react/dist/csr/Gear";
import { LockKeyIcon } from "@phosphor-icons/react/dist/csr/LockKey";
import { SignOutIcon } from "@phosphor-icons/react/dist/csr/SignOut";
import { CaretRightIcon } from "@phosphor-icons/react/dist/csr/CaretRight";
import { getSavedMaterials } from "../../api/library";
import { getAuthorMaterials } from "../../api/materialWorkspace";
import { useApiResource } from "../../hooks/useApiResource";
import { useMobileViewport } from "../../hooks/useMobileViewport";
import { useViewer } from "../../hooks/useViewer";
import { useTheme } from "../../hooks/useTheme";
import type { User } from "../../types/viewer";
import styles from "./MobileProfileOverview.module.css";

export function MobileProfileStats({ user }: { user: User }) {
  const mobile = useMobileViewport();
  const { libraryVersion } = useViewer();
  const load = useCallback(
    async (signal: AbortSignal) => {
      // A save change invalidates the counts even when the viewport stays mobile.
      void libraryVersion;
      if (!mobile) {
        return null;
      }
      const [saved, materials] = await Promise.all([
        getSavedMaterials(1, signal),
        getAuthorMaterials(
          { page: 1, q: "", state: "active", order: "newest" },
          signal,
        ),
      ]);
      return { saved: saved.total, materials: materials.total };
    },
    [mobile, libraryVersion],
  );
  const resource = useApiResource(load, "Не удалось загрузить счётчики.");
  const third =
    user.role === "student"
      ? user.course
      : user.memberships.filter((membership) => membership.state === "active")
          .length;

  return (
    <div className={styles.mobile}>
      <nav className={styles.stats} aria-label="Быстрые переходы профиля">
        <Link to="/library">
          <BookOpenIcon size={26} aria-hidden="true" />
          <strong>{resource.result?.saved ?? "—"}</strong>
          <span>
            Сохранено
            <br />в библиотеке
          </span>
        </Link>
        <Link to="/author/materials">
          <FileTextIcon size={26} aria-hidden="true" />
          <strong>{resource.result?.materials ?? "—"}</strong>
          <span>
            Мои
            <br />
            материалы
          </span>
        </Link>
        <Link to="/settings#institution-membership">
          <GraduationCapIcon size={26} aria-hidden="true" />
          <strong>{third ?? "—"}</strong>
          <span>
            {user.role === "student" ? "Курс обучения" : "Учреждений"}
          </span>
        </Link>
      </nav>
      {resource.error && (
        <p className={styles.error} role="status">
          {resource.error}
          <button type="button" onClick={resource.retry}>
            Повторить
          </button>
        </p>
      )}
    </div>
  );
}

export function MobileProfileActions() {
  const { signOut } = useViewer();
  const { theme } = useTheme();
  const navigate = useNavigate();
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const logout = async () => {
    if (busy) {
      return;
    }
    setBusy(true);
    setError("");
    try {
      await signOut();
      navigate("/catalog", { replace: true });
    } catch (reason) {
      setError(
        reason instanceof Error
          ? reason.message
          : "Не удалось выйти из аккаунта.",
      );
      setBusy(false);
    }
  };

  return (
    <div className={styles.mobile}>
      <Link className={styles.actionCard} to="/settings">
        <LockKeyIcon size={24} aria-hidden="true" />
        <span>
          <strong>Аккаунт и безопасность</strong>
          <small>Пароль и управление сессиями</small>
        </span>
        <CaretRightIcon size={20} aria-hidden="true" />
      </Link>
      <Link className={styles.actionCard} to="/settings">
        <GearIcon size={24} aria-hidden="true" />
        <span>
          <strong>Настройки</strong>
          <small>
            {theme === "dark" ? "Тёмная тема" : "Светлая тема"} · Уведомления
          </small>
        </span>
        <CaretRightIcon size={20} aria-hidden="true" />
      </Link>
      <button
        className={styles.logout}
        type="button"
        disabled={busy}
        onClick={() => void logout()}
      >
        <SignOutIcon size={22} aria-hidden="true" />
        {busy ? "Выходим…" : "Выйти из аккаунта"}
      </button>
      {error && <p role="alert">{error}</p>}
    </div>
  );
}
