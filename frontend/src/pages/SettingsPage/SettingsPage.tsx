import { lazy, Suspense, useState } from "react";
import { Link, useLocation } from "react-router-dom";
import { ThemeToggle } from "../../components/common/ThemeToggle/ThemeToggle";
import { Button } from "../../components/common/Button/Button";
import { IdentityRequests } from "../../components/account/IdentityRequests";
import { logoutOthers } from "../../api/auth";
import { useViewer } from "../../hooks/useViewer";
import styles from "./SettingsPage.module.css";
import { ProfileLayout } from "../ProfilePage/ProfileLayout";
import { SettingsPreferences } from "./SettingsPreferences";
import { EnvelopeSimpleIcon } from "@phosphor-icons/react/dist/csr/EnvelopeSimple";
import { LockKeyIcon } from "@phosphor-icons/react/dist/csr/LockKey";
import { SunIcon } from "@phosphor-icons/react/dist/csr/Sun";
import { DevicesIcon } from "@phosphor-icons/react/dist/csr/Devices";
import { PasswordForm } from "../../components/account/security/PasswordForm";
import { useNavigationContext } from "../../hooks/useNavigationContext";
const InstitutionRegistration = lazy(async () => ({
  default: (await import("../../components/account/InstitutionRegistration"))
    .InstitutionRegistration,
}));

export function SettingsPage() {
  const { linkTo } = useNavigationContext();
  const location = useLocation();
  const { viewer } = useViewer();
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  if (viewer.kind !== "authenticated") {
    return null;
  }
  const closeOtherSessions = async () => {
    if (busy) {
      return;
    }
    setBusy(true);
    setNotice(null);
    try {
      await logoutOthers();
      setNotice("Другие сессии отозваны.");
    } catch (reason) {
      setNotice(
        reason instanceof Error
          ? reason.message
          : "Не удалось завершить сессии",
      );
    } finally {
      setBusy(false);
    }
  };
  return (
    <ProfileLayout user={viewer.user}>
      <div className={styles.page}>
        <div className={styles.intro}>
          <h1>Настройки</h1>
          <p>Аккаунт, оформление и безопасность в одном месте.</p>
        </div>
        <div className={styles.grid}>
          <section className={styles.card}>
            <header className={styles.cardHeading}>
              <h2>Аккаунт</h2>
              <Link to={linkTo("/profile?edit=1")}>Изменить профиль</Link>
            </header>
            <div className={styles.row}>
              <EnvelopeSimpleIcon size={21} aria-hidden="true" />
              <span>Email</span>
              <strong>{viewer.user.email}</strong>
            </div>
            <div className={styles.row}>
              <LockKeyIcon size={21} aria-hidden="true" />
              <span>Пароль</span>
              <span>Установлен</span>
            </div>
            <details className={styles.password}>
              <summary>Сменить пароль</summary>
              <PasswordForm />
            </details>
          </section>
          <SettingsPreferences kind="notifications" />
          <SettingsPreferences kind="privacy" />
          <section className={styles.card}>
            <header className={styles.cardHeading}>
              <h2>Оформление и доступ</h2>
            </header>
            <div className={styles.row}>
              <SunIcon size={21} aria-hidden="true" />
              <span>Тема интерфейса</span>
              <ThemeToggle />
            </div>
            <div className={styles.row}>
              <span className={styles.textIcon}>Аа</span>
              <span>Размер интерфейса</span>
              <span>Стандартный</span>
            </div>
            <p className={styles.hint}>Тема применяется ко всему приложению.</p>
          </section>
          <section className={`${styles.card} ${styles.fullWidth}`}>
            <header className={styles.cardHeading}>
              <h2>Сессии и безопасность</h2>
              <DevicesIcon size={22} aria-hidden="true" />
            </header>
            <div className={styles.sessionRow}>
              <div>
                <strong>Вход на других устройствах</strong>
                <p>
                  Завершите остальные сессии, сохранив вход на этом устройстве.
                </p>
              </div>
              <Button
                variant="secondary"
                disabled={busy}
                onClick={() => void closeOtherSessions()}
              >
                Завершить остальные сессии
              </Button>
            </div>
            {notice && (
              <p className={styles.notice} role="status">
                {notice}
              </p>
            )}
          </section>
        </div>
        <details
          className={styles.disclosure}
          id="institution-membership"
          open={location.hash === "#institution-membership"}
        >
          <summary>
            Учреждения и авторство{" "}
            <span>Заявки, членство и связь с автором</span>
          </summary>
          <IdentityRequests />
        </details>
        <details className={styles.disclosure}>
          <summary>
            Регистрация учреждения <span>Создать новое учреждение</span>
          </summary>
          <Suspense
            fallback={<p role="status">Загружаем регистрацию учреждения…</p>}
          >
            <InstitutionRegistration />
          </Suspense>
        </details>
      </div>
    </ProfileLayout>
  );
}
