import { useEffect, useRef, useState, type FormEvent } from "react";
import { UserIcon } from "@phosphor-icons/react/dist/csr/User";
import { updateProfile } from "../../../api/profile";
import type { ProfileInput, User } from "../../../types/viewer";
import { roleLabels } from "../../../utils/viewerLabels";
import { Button } from "../../common/Button/Button";
import styles from "./ProfileEditor.module.css";

interface ProfileEditorProps {
  user: User;
  onSaved: (user: User) => void;
  onCancel: () => void;
}

export function ProfileEditor({ user, onSaved, onCancel }: ProfileEditorProps) {
  const [input, setInput] = useState<ProfileInput>({
    firstName: user.firstName,
    lastName: user.lastName,
    course: user.course,
    group: user.group,
    biography: user.biography,
  });
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const request = useRef<AbortController | null>(null);
  const firstField = useRef<HTMLInputElement>(null);

  useEffect(() => {
    firstField.current?.focus();
    return () => request.current?.abort();
  }, []);

  const save = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (saving) {
      return;
    }
    if (!input.firstName.trim()) {
      setError("Укажите имя.");
      firstField.current?.focus();
      return;
    }
    const controller = new AbortController();
    request.current = controller;
    setSaving(true);
    setError(null);
    try {
      const updated = await updateProfile(input, controller.signal);
      if (!controller.signal.aborted) {
        onSaved(updated);
      }
    } catch (reason) {
      if (!controller.signal.aborted) {
        setError(
          reason instanceof Error
            ? reason.message
            : "Не удалось сохранить профиль.",
        );
      }
    } finally {
      if (!controller.signal.aborted) {
        setSaving(false);
      }
    }
  };

  return (
    <form
      className={styles.form}
      onSubmit={save}
      aria-label="Редактирование профиля"
    >
      <div className={styles.identity}>
        <span className={styles.avatar}>
          <UserIcon size={42} weight="fill" aria-hidden="true" />
        </span>
        <div>
          <strong>{user.name}</strong>
          <p>{roleLabels[user.role]} · Фото профиля будет доступно позже</p>
        </div>
      </div>
      <fieldset disabled={saving} className={styles.fields}>
        <section className={styles.card}>
          <h2>Личная информация</h2>
          <div className={styles.columns}>
            <label>
              Имя
              <input
                ref={firstField}
                required
                maxLength={64}
                autoComplete="given-name"
                value={input.firstName}
                onChange={(event) =>
                  setInput({ ...input, firstName: event.target.value })
                }
              />
            </label>
            <label>
              Фамилия
              <input
                maxLength={64}
                autoComplete="family-name"
                value={input.lastName}
                onChange={(event) =>
                  setInput({ ...input, lastName: event.target.value })
                }
              />
            </label>
          </div>
          <label>
            О себе
            <textarea
              rows={5}
              maxLength={2000}
              value={input.biography}
              onChange={(event) =>
                setInput({ ...input, biography: event.target.value })
              }
            />
          </label>
          <small className={styles.counter}>
            {input.biography.length} / 2000
          </small>
        </section>
        {user.role === "student" && (
          <section className={styles.card}>
            <h2>Учебная информация</h2>
            <div className={styles.columns}>
              <label>
                Курс
                <select
                  value={input.course ?? ""}
                  onChange={(event) =>
                    setInput({
                      ...input,
                      course: event.target.value
                        ? Number(event.target.value)
                        : null,
                    })
                  }
                >
                  <option value="">Не указан</option>
                  {[1, 2, 3, 4, 5, 6].map((course) => (
                    <option key={course} value={course}>
                      {course} курс
                    </option>
                  ))}
                </select>
              </label>
              <label>
                Группа
                <input
                  maxLength={30}
                  value={input.group}
                  onChange={(event) =>
                    setInput({ ...input, group: event.target.value })
                  }
                />
              </label>
            </div>
            <p>Учреждения и заявки доступны в настройках аккаунта.</p>
          </section>
        )}
        <section className={styles.card}>
          <h2>Контактная информация</h2>
          <label>
            Email
            <input type="email" readOnly value={user.email} />
          </label>
          <p>Используется для входа. Изменение адреса пока недоступно.</p>
        </section>
      </fieldset>
      {error && (
        <p className={styles.error} role="alert">
          {error}
        </p>
      )}
      <div className={styles.actions}>
        <Button type="submit" variant="primary" loading={saving}>
          Сохранить изменения
        </Button>
        <Button type="button" onClick={onCancel} disabled={saving}>
          Отмена
        </Button>
      </div>
    </form>
  );
}
