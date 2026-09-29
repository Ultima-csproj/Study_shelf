import { useState } from 'react'
import { Link } from 'react-router-dom'
import { LockKeyIcon } from '@phosphor-icons/react/dist/csr/LockKey'
import type { DisciplineSummary } from '../../../types/discipline'
import { useViewer } from '../../../hooks/useViewer'
import styles from './DisciplineHeader.module.css'
export function DisciplineHeader({
  discipline,
}: {
  discipline: DisciplineSummary
}) {
  const [expanded, setExpanded] = useState(false)
  const { viewer } = useViewer()
  const teacher = discipline.teacher
  const descriptionClassName = `${styles.description} ${expanded ? styles.expanded : ''}`
  const descriptionAction = expanded ? 'Свернуть' : 'Подробнее'
  const compactDescription = expanded
    ? discipline.description
    : (discipline.summary ?? discipline.description)
  return (
    <div className={styles.header}>
      <nav
        aria-label="Хлебные крошки"
        className={styles.breadcrumbs}
      >
        <Link to="/disciplines/database">Дисциплины</Link>
        <span aria-hidden="true">/</span>
        <span>{discipline.name}</span>
      </nav>
      <h1>{discipline.name}</h1>
      <p
        id="discipline-description"
        className={descriptionClassName}
      >
        <span className={styles.fullDescription}>{discipline.description}</span>
        <span className={styles.compactDescription}>{compactDescription}</span>
      </p>
      <button
        type="button"
        className={styles.more}
        aria-expanded={expanded}
        aria-controls="discipline-description"
        onClick={() => setExpanded((value) => !value)}
      >
        {descriptionAction}
      </button>
      <div className={styles.meta}>
        <span>
          {discipline.course} курс <span aria-hidden="true">·</span>{' '}
          {discipline.semester} семестр
        </span>
        <span className={styles.teacher}>{teacher?.shortName}</span>
      </div>
      {viewer.kind === 'guest' && (
        <p className={styles.hint}>
          <LockKeyIcon
            size={16}
            weight="fill"
            aria-hidden="true"
          />
          <span className={styles.desktopHint}>
            Для сохранения материалов потребуется вход.
          </span>
          <span className={styles.mobileHint}>
            Сохранение материалов — после входа.
          </span>
        </p>
      )}
    </div>
  )
}
