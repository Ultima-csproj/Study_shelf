import { ArrowCounterClockwiseIcon } from "@phosphor-icons/react/dist/csr/ArrowCounterClockwise";
import type { CollectionRequest } from "../../../types/collection";
import type { Discipline } from "../../../types/discipline";
import styles from "./CollectionFilters.module.css";

interface CollectionFiltersProps {
  request: CollectionRequest;
  disciplines: Discipline[];
  courses: number[];
  onChange: (request: CollectionRequest) => void;
  onReset: () => void;
}

export function CollectionFilters({
  request,
  disciplines,
  courses,
  onChange,
  onReset,
}: CollectionFiltersProps) {
  const toggleDiscipline = (id: number) => {
    let disciplineIds: number[];

    if (request.disciplineIds.includes(id)) {
      disciplineIds = request.disciplineIds.filter((value) => value !== id);
    } else {
      disciplineIds = [...request.disciplineIds, id];
    }

    const nextRequest: CollectionRequest = {
      ...request,
      disciplineIds,
    };

    onChange(nextRequest);
  };

  const toggleCourse = (course: number) => {
    let courses: number[];

    if (request.courses.includes(course)) {
      courses = request.courses.filter((value) => value !== course);
    } else {
      courses = [...request.courses, course];
    }

    const nextRequest: CollectionRequest = {
      ...request,
      courses,
    };

    onChange(nextRequest);
  };

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
      <button type="button" className={styles.reset} onClick={onReset}>
        <ArrowCounterClockwiseIcon size={20} aria-hidden="true" />
        Сбросить фильтры
      </button>
    </div>
  );
}
