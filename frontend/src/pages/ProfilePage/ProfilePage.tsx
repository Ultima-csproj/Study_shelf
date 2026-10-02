import {
  useEffect,
  useRef,
  useState,
  type ChangeEvent,
  type FormEvent,
  type ReactNode,
} from "react";
import { Link } from "react-router-dom";
import { PencilSimpleIcon } from "@phosphor-icons/react/dist/csr/PencilSimple";
import { ArrowRightIcon } from "@phosphor-icons/react/dist/csr/ArrowRight";
import { ArrowSquareOutIcon } from "@phosphor-icons/react/dist/csr/ArrowSquareOut";
import { useViewer } from "../../hooks/useViewer";
import { updateProfile } from "../../api/profile";
import { roleLabels } from "../../utils/viewerLabels";
import type { ProfileInput, User } from "../../types/viewer";
import styles from "./ProfilePage.module.css";

interface ProfileContentProps {
  user: User;
  onSaved: (user: User) => void;
}

export function ProfilePage() {
  const { viewer, updateUser } = useViewer();

  if (viewer.kind !== "authenticated") {
    return null;
  }

  return <ProfileContent user={viewer.user} onSaved={updateUser} />;
}

function ProfileContent({
  user,
  onSaved,
}: ProfileContentProps) {
  const [editing, setEditing] = useState(false);
  const [input, setInput] = useState<ProfileInput>({
    firstName: user.firstName,
    lastName: user.lastName,
    course: user.course,
    group: user.group,
  });
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState("");
  const request = useRef<AbortController | null>(null);
  const editButton = useRef<HTMLButtonElement>(null);
  const firstNameInput = useRef<HTMLInputElement>(null);
  const wasEditing = useRef(false);
  const student = user.role === "student";
  let courseValue = "";

  if (input.course !== null) {
    courseValue = String(input.course);
  }

  let courseLabel = "Не указан";

  if (user.course !== null) {
    courseLabel = `${user.course} курс`;
  }

  let groupLabel = user.group;

  if (!groupLabel) {
    groupLabel = "Не указана";
  }

  useEffect(() => {
    return () => {
      request.current?.abort();
    };
  }, []);

  useEffect(() => {
    if (editing) {
      firstNameInput.current?.focus();
    } else if (wasEditing.current) {
      editButton.current?.focus();
    }
    wasEditing.current = editing;
  }, [editing]);

  const startEditing = () => {
    setInput({
      firstName: user.firstName,
      lastName: user.lastName,
      course: user.course,
      group: user.group,
    });
    setError(null);
    setNotice("");
    setEditing(true);
  };

  const cancelEditing = () => {
    request.current?.abort();
    setSaving(false);
    setError(null);
    setEditing(false);
  };

  const handleFirstNameChange = (event: ChangeEvent<HTMLInputElement>) => {
    const firstName = event.target.value;

    setInput((currentInput) => {
      return { ...currentInput, firstName };
    });
  };

  const handleLastNameChange = (event: ChangeEvent<HTMLInputElement>) => {
    const lastName = event.target.value;

    setInput((currentInput) => {
      return { ...currentInput, lastName };
    });
  };

  const handleCourseChange = (event: ChangeEvent<HTMLSelectElement>) => {
    let course: number | null = null;

    if (event.target.value) {
      course = Number(event.target.value);
    }

    setInput((currentInput) => {
      return { ...currentInput, course };
    });
  };

  const handleGroupChange = (event: ChangeEvent<HTMLInputElement>) => {
    const group = event.target.value;

    setInput((currentInput) => {
      return { ...currentInput, group };
    });
  };

  const save = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (saving) {
      return;
    }

    const controller = new AbortController();
    request.current = controller;
    setSaving(true);
    setError(null);

    try {
      const updated = await updateProfile(user, input, controller.signal);

      if (!controller.signal.aborted) {
        onSaved(updated);
        setNotice("Изменения профиля сохранены в текущей демо-сессии.");
        setEditing(false);
      }
    } catch (reason: unknown) {
      if (!controller.signal.aborted) {
        let message = "Не удалось сохранить профиль.";

        if (reason instanceof Error) {
          message = reason.message;
        }

        setError(message);
      }
    } finally {
      if (!controller.signal.aborted) {
        setSaving(false);
      }
    }
  };

  let studentFields: ReactNode = null;

  if (student) {
    studentFields = (
      <section className={styles.section}>
        <h2>Обучение</h2>
        <label htmlFor="profile-course">Курс</label>
        <select
          id="profile-course"
          value={courseValue}
          onChange={handleCourseChange}
        >
          <option value="">Не указан</option>
          {[1, 2, 3, 4, 5, 6].map((course) => (
            <option key={course} value={course}>
              {course} курс
            </option>
          ))}
        </select>
        <label htmlFor="profile-group">Группа</label>
        <input
          id="profile-group"
          maxLength={30}
          value={input.group}
          onChange={handleGroupChange}
        />
      </section>
    );
  }

  let errorMessage: ReactNode = null;

  if (error) {
    errorMessage = <p role="alert">{error}</p>;
  }

  let saveButtonLabel = "Сохранить изменения";

  if (saving) {
    saveButtonLabel = "Сохраняем…";
  }

  let profileFields: ReactNode;

  if (editing) {
    profileFields = (
      <form
        onSubmit={save}
        className={styles.form}
        aria-label="Редактирование профиля"
      >
        <section className={styles.section}>
          <h2>Личная информация</h2>
          <label htmlFor="profile-first-name">Имя</label>
          <input
            ref={firstNameInput}
            id="profile-first-name"
            autoComplete="given-name"
            required
            maxLength={64}
            value={input.firstName}
            onChange={handleFirstNameChange}
          />
          <label htmlFor="profile-last-name">Фамилия</label>
          <input
            id="profile-last-name"
            autoComplete="family-name"
            required
            maxLength={64}
            value={input.lastName}
            onChange={handleLastNameChange}
          />
        </section>
        {studentFields}
        {errorMessage}
        <div className={styles.actions}>
          <button type="submit" className={styles.primary} disabled={saving}>
            {saveButtonLabel}
          </button>
          <button
            type="button"
            className={styles.secondary}
            onClick={cancelEditing}
          >
            Отмена
          </button>
        </div>
      </form>
    );
  } else {
    let studentSummary: ReactNode = null;

    if (student) {
      studentSummary = (
        <section className={styles.section}>
          <h2>Обучение</h2>
          <dl>
            <dt>Курс</dt>
            <dd>{courseLabel}</dd>
            <dt>Группа</dt>
            <dd>{groupLabel}</dd>
          </dl>
        </section>
      );
    }

    profileFields = (
      <>
        <section className={styles.section}>
          <h2>Личная информация</h2>
          <dl>
            <dt>Имя</dt>
            <dd>{user.firstName}</dd>
            <dt>Фамилия</dt>
            <dd>{user.lastName}</dd>
          </dl>
        </section>
        {studentSummary}
      </>
    );
  }

  let editProfileButton: ReactNode = null;

  if (!editing) {
    editProfileButton = (
      <button
        ref={editButton}
        type="button"
        className={styles.primary}
        onClick={startEditing}
      >
        <PencilSimpleIcon size={20} aria-hidden="true" />
        Редактировать профиль
      </button>
    );
  }

  let profileNotice: ReactNode = null;

  if (notice) {
    profileNotice = (
      <p role="status" className={styles.notice}>
        {notice}
      </p>
    );
  }

  return (
    <div className={styles.page}>
      <nav aria-label="Хлебные крошки" className={styles.breadcrumbs}>
        <span>Личная зона</span>
        <span aria-hidden="true">/</span>
        <span>Профиль</span>
      </nav>
      <div className={styles.heading}>
        <div>
          <h1>Мой профиль</h1>
          <p>Личная информация и данные об обучении.</p>
        </div>
        {editProfileButton}
      </div>
      <div className={styles.identity}>
        <span className={styles.avatar} aria-hidden="true">
          {user.initials}
        </span>
        <div>
          <h2>{user.name}</h2>
          <p className={styles.role}>{roleLabels[user.role]}</p>
          <p className={styles.privacy}>Информация видна только вам.</p>
        </div>
      </div>
      {profileNotice}
      {profileFields}
      <Link to="/settings" className={styles.settingsLink}>
        <ArrowSquareOutIcon size={20} aria-hidden="true" />
        <span>Email, пароль и безопасность — в настройках аккаунта</span>
        <ArrowRightIcon size={18} aria-hidden="true" />
      </Link>
    </div>
  );
}
