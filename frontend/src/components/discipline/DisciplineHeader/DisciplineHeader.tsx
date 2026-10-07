import { useState } from "react";
import type { ReactNode } from "react";
import { Link } from "react-router-dom";
import { LockKeyIcon } from "@phosphor-icons/react/dist/csr/LockKey";
import type { DisciplineSummary } from "../../../types/discipline";
import { useViewer } from "../../../hooks/useViewer";
import styles from "./DisciplineHeader.module.css";

interface DisciplineHeaderProps {
  discipline: DisciplineSummary;
}

export function DisciplineHeader({
  discipline,
}: DisciplineHeaderProps) {
  const [expanded, setExpanded] = useState(false);
  const { viewer } = useViewer();
  const responsible = discipline.responsible.map((person) => person.name).join(", ");
  const courses = discipline.courses.join(", ");
  let descriptionClassName = styles.description;
  let descriptionAction = "Подробнее";

  if (expanded) {
    descriptionClassName = `${styles.description} ${styles.expanded}`;
    descriptionAction = "Свернуть";
  }

  let compactDescription = discipline.summary ?? discipline.description;

  if (expanded) {
    compactDescription = discipline.description;
  }

  const handleDescriptionToggle = () => {
    setExpanded((currentExpanded) => {
      if (currentExpanded) {
        return false;
      }

      return true;
    });
  };

  let guestHint: ReactNode = null;

  if (viewer.kind === "guest") {
    guestHint = (
      <p className={styles.hint}>
        <LockKeyIcon size={16} weight="fill" aria-hidden="true" />
        <span className={styles.desktopHint}>
          Для сохранения материалов потребуется вход.
        </span>
        <span className={styles.mobileHint}>
          Сохранение материалов — после входа.
        </span>
      </p>
    );
  }

  return (
    <div className={styles.header}>
      <nav aria-label="Хлебные крошки" className={styles.breadcrumbs}>
        <Link to="/disciplines">Дисциплины</Link>
        <span aria-hidden="true">/</span>
        <span>{discipline.name}</span>
      </nav>
      <h1>{discipline.name}</h1>
      <p id="discipline-description" className={descriptionClassName}>
        <span className={styles.fullDescription}>{discipline.description}</span>
        <span className={styles.compactDescription}>{compactDescription}</span>
      </p>
      <button
        type="button"
        className={styles.more}
        aria-expanded={expanded}
        aria-controls="discipline-description"
        onClick={handleDescriptionToggle}
      >
        {descriptionAction}
      </button>
      <div className={styles.meta}>
        <span>
          {discipline.institution.name}
        </span>
        {courses && <span>Курсы: {courses}</span>}
        {responsible && <span className={styles.teacher}>Ответственные: {responsible}</span>}
      </div>
      {guestHint}
    </div>
  );
}
