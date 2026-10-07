import { useState, type FormEvent } from "react";
import { Link, Navigate, useSearchParams } from "react-router-dom";
import { useViewer } from "../../hooks/useViewer";
import { safeRedirect } from "../../utils/redirect";
import styles from "../InformationPage/InformationPage.module.css";
import formStyles from "../../components/account/AccountForm.module.css";

export function LoginPage() {
  const [params] = useSearchParams();
  const { viewer, signIn, refreshViewer } = useViewer();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [firstName, setFirstName] = useState("");
  const [lastName, setLastName] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const register = params.get("mode") === "register";
  const redirect = safeRedirect(params.get("redirect"));
  const modeParams = new URLSearchParams({ redirect });
  if (!register) {
    modeParams.set("mode", "register");
  }

  if (viewer.kind === "authenticated") {
    return <Navigate to={redirect} replace />;
  }

  const submit = async (event: FormEvent) => {
    event.preventDefault();
    if (busy) {
      return;
    }
    setBusy(true);
    setError(null);
    try {
      if (register) {
        await signIn({ email, password, firstName, lastName }, true);
      } else {
        await signIn({ email, password }, false);
      }
      setPassword("");
    } catch (reason) {
      setError(reason instanceof Error ? reason.message : "Не удалось войти");
    } finally {
      setBusy(false);
    }
  };
  const title = register ? "Создать аккаунт" : "Войти";
  const buttonLabel = busy ? "Подождите…" : title;
  const passwordComplete = register ? "new-password" : "current-password";
  let nameFields = null;
  if (register) {
    nameFields = (
      <>
        <label htmlFor="auth-first-name">Имя</label>
        <input
          id="auth-first-name"
          required
          maxLength={64}
          autoComplete="given-name"
          value={firstName}
          onChange={(event) => setFirstName(event.target.value)}
        />
        <label htmlFor="auth-last-name">Фамилия</label>
        <input
          id="auth-last-name"
          maxLength={64}
          autoComplete="family-name"
          value={lastName}
          onChange={(event) => setLastName(event.target.value)}
        />
      </>
    );
  }

  return (
    <main id="main-content" className={styles.page}>
      <Link className={styles.back} to="/catalog">
        ← Вернуться к просмотру
      </Link>
      <h1>{title}</h1>
      <p>
        Каталог доступен без аккаунта. После входа можно сохранять материалы и
        создавать личные подборки.
      </p>
      {viewer.kind === "error" && (
        <p role="alert">
          {viewer.message}{" "}
          <button type="button" onClick={() => void refreshViewer()}>
            Повторить
          </button>
        </p>
      )}
      <form className={formStyles.form} onSubmit={submit}>
        <fieldset disabled={busy || viewer.kind === "loading"}>
          {nameFields}
          <label htmlFor="auth-email">Email</label>
          <input
            id="auth-email"
            type="email"
            required
            maxLength={254}
            autoComplete="username"
            value={email}
            onChange={(event) => setEmail(event.target.value)}
          />
          <label htmlFor="auth-password">Пароль</label>
          <input
            id="auth-password"
            type="password"
            required
            minLength={register ? 12 : undefined}
            maxLength={1024}
            autoComplete={passwordComplete}
            value={password}
            onChange={(event) => setPassword(event.target.value)}
          />
          {register && (
            <p>
              Не менее 12 символов. Принадлежность к учреждению можно
              подтвердить после регистрации.
            </p>
          )}
          {error && <p role="alert">{error}</p>}
          <button className={styles.primary} type="submit">
            {buttonLabel}
          </button>
        </fieldset>
      </form>
      <Link to={`/login?${modeParams}`}>
        {register ? "Уже есть аккаунт? Войти" : "Создать аккаунт"}
      </Link>
    </main>
  );
}
