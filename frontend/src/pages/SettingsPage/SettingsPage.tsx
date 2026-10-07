import { useState, type FormEvent } from "react";
import { ThemeToggle } from "../../components/common/ThemeToggle/ThemeToggle";
import { IdentityRequests } from "../../components/account/IdentityRequests";
import { changePassword, logoutOthers } from "../../api/auth";
import { useViewer } from "../../hooks/useViewer";
import styles from "./SettingsPage.module.css";
import formStyles from "../../components/account/AccountForm.module.css";

export function SettingsPage() {
  const { viewer, refreshViewer } = useViewer();
  const [currentPassword, setCurrentPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  if (viewer.kind !== "authenticated") {
    return null;
  }
  const submit = async (event: FormEvent) => {
    event.preventDefault();
    if (busy) {
      return;
    }
    setBusy(true);
    setNotice(null);
    try {
      await changePassword(currentPassword, newPassword);
      setCurrentPassword("");
      setNewPassword("");
      await refreshViewer();
    } catch (reason) {
      setNotice(
        reason instanceof Error ? reason.message : "Не удалось изменить пароль",
      );
    } finally {
      setBusy(false);
    }
  };
  const closeOtherSessions = async () => {
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
    <div className={styles.page}>
      <h1>Настройки</h1>
      <section>
        <h2>Внешний вид</h2>
        <div className={styles.appearance}>
          <p>Светлая или тёмная тема</p>
          <ThemeToggle />
        </div>
      </section>
      <section>
        <h2>Аккаунт</h2>
        <p>ID аккаунта: {viewer.user.id}</p>
        <p>Email: {viewer.user.email}</p>
        <p>Основной автор: #{viewer.user.primaryContributorId}</p>
        <p>
          Подтверждение email и восстановление пароля через почту будут
          подключены отдельно.
        </p>
        <form className={formStyles.form} onSubmit={submit}>
          <fieldset disabled={busy}>
            <label htmlFor="current-password">Текущий пароль</label>
            <input
              id="current-password"
              type="password"
              required
              autoComplete="current-password"
              maxLength={1024}
              value={currentPassword}
              onChange={(event) => setCurrentPassword(event.target.value)}
            />
            <label htmlFor="new-password">
              Новый пароль — минимум 12 символов
            </label>
            <input
              id="new-password"
              type="password"
              required
              minLength={12}
              maxLength={1024}
              autoComplete="new-password"
              value={newPassword}
              onChange={(event) => setNewPassword(event.target.value)}
            />
            <p>
              После смены пароля потребуется войти заново на всех устройствах.
            </p>
            <button type="submit">Изменить пароль</button>
          </fieldset>
        </form>
        <button
          type="button"
          disabled={busy}
          onClick={() => void closeOtherSessions()}
        >
          Выйти на остальных устройствах
        </button>
        {notice && <p role="status">{notice}</p>}
      </section>
      <IdentityRequests />
    </div>
  );
}
