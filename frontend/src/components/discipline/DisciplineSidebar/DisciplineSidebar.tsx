import { Link } from 'react-router-dom'
import type { DisciplineSummary } from '../../../types/discipline'
import styles from './DisciplineSidebar.module.css'
export function DisciplineSidebar({
  disciplines,
  selectedId,
}: {
  disciplines: DisciplineSummary[]
  selectedId: number
}) {
  return (
    <aside className={styles.sidebar}>
      <nav aria-label="Дисциплины">
        <h2>Дисциплины</h2>
        {disciplines.map((discipline) => {
          const selected = discipline.id === selectedId
          const className = selected ? styles.selected : ''
          const currentPage = selected ? 'page' : undefined
          return (
            <Link
              key={discipline.id}
              to={`/disciplines/${discipline.slug}`}
              className={className}
              aria-current={currentPage}
            >
              <span>{discipline.name}</span>
              <span className={styles.count}>{discipline.materialCount}</span>
            </Link>
          )
        })}
      </nav>
    </aside>
  )
}
