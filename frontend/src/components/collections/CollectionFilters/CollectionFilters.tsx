import { ArrowCounterClockwiseIcon } from '@phosphor-icons/react/dist/csr/ArrowCounterClockwise'
import type { CollectionRequest } from '../../../types/collection'
import type { Discipline } from '../../../types/discipline'
import styles from './CollectionFilters.module.css'

export function CollectionFilters({
  request,
  disciplines,
  courses,
  onChange,
  onReset,
}: {
  request: CollectionRequest
  disciplines: Discipline[]
  courses: number[]
  onChange: (request: CollectionRequest) => void
  onReset: () => void
}) {
  const toggleDiscipline = (id: number) => {
    const disciplineIds = request.disciplineIds.includes(id)
      ? request.disciplineIds.filter((value) => value !== id)
      : [...request.disciplineIds, id]

    onChange({ ...request, disciplineIds })
  }

  const toggleCourse = (course: number) => {
    const courses = request.courses.includes(course)
      ? request.courses.filter((value) => value !== course)
      : [...request.courses, course]

    onChange({ ...request, courses })
  }

  return (
    <div className={styles.filters}>
      <h2>Фильтры</h2>
      <fieldset>
        <legend>Дисциплина</legend>
        {disciplines.map((discipline) => (
          <label key={discipline.id}>
            <input
              type="checkbox"
              checked={request.disciplineIds.includes(discipline.id)}
              onChange={() => toggleDiscipline(discipline.id)}
            />
            <span>{discipline.name}</span>
          </label>
        ))}
      </fieldset>
      <fieldset>
        <legend>Курс</legend>
        {courses.map((course) => (
          <label key={course}>
            <input
              type="checkbox"
              checked={request.courses.includes(course)}
              onChange={() => toggleCourse(course)}
            />
            <span>{course} курс</span>
          </label>
        ))}
      </fieldset>
      <button
        type="button"
        className={styles.reset}
        onClick={onReset}
      >
        <ArrowCounterClockwiseIcon
          size={20}
          aria-hidden="true"
        />
        Сбросить фильтры
      </button>
    </div>
  )
}
