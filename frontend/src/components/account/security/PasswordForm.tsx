import { useState, type FormEvent } from "react";
import { changePassword } from "../../../api/auth";
import { useViewer } from "../../../hooks/useViewer";
import { Button } from "../../common/Button/Button";
import styles from "../../common/Form/Form.module.css";

export function PasswordForm() {
  const { refreshViewer } = useViewer();
  const [current, setCurrent] = useState("");
  const [password, setPassword] = useState("");
  const [confirmation, setConfirmation] = useState("");
  const [visible, setVisible] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (busy) {
      return;
    }
    if (password !== confirmation) {
      setError("Новый пароль и подтверждение не совпадают.");
      return;
    }
    setBusy(true);
    setError(null);
    try {
      await changePassword(current, password);
      setCurrent("");
      setPassword("");
      setConfirmation("");
      await refreshViewer();
    } catch (reason) {
      setError(
        reason instanceof Error
          ? reason.message
          : "Не удалось изменить пароль.",
      );
    } finally {
      setBusy(false);
    }
  };

  return (
    <form className={styles.form} onSubmit={submit} aria-label="Смена пароля">
      <fieldset disabled={busy}>
        <label htmlFor="current-password">Текущий пароль</label>
        <input
          id="current-password"
          type={visible ? "text" : "password"}
          autoComplete="current-password"
          required
          maxLength={1024}
          value={current}
          onChange={(event) => setCurrent(event.target.value)}
        />
        <label htmlFor="new-password">Новый пароль — минимум 12 символов</label>
        <input
          id="new-password"
          type={visible ? "text" : "password"}
          autoComplete="new-password"
          required
          minLength={12}
          maxLength={1024}
          value={password}
          onChange={(event) => setPassword(event.target.value)}
        />
        <label htmlFor="confirm-password">Повторите новый пароль</label>
        <input
          id="confirm-password"
          type={visible ? "text" : "password"}
          autoComplete="new-password"
          required
          minLength={12}
          maxLength={1024}
          value={confirmation}
          onChange={(event) => setConfirmation(event.target.value)}
          aria-invalid={error && password !== confirmation ? true : undefined}
        />
        <Button
          type="button"
          variant="ghost"
          aria-pressed={visible}
          onClick={() => setVisible(!visible)}
        >
          {visible ? "Скрыть пароли" : "Показать пароли"}
        </Button>
        <p>После смены пароля потребуется войти заново на всех устройствах.</p>
        {error && <p role="alert">{error}</p>}
        <Button type="submit" variant="primary" loading={busy}>
          Изменить пароль
        </Button>
      </fieldset>
    </form>
  );
}
