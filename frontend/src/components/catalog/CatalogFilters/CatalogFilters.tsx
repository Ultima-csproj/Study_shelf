import { useState } from "react";
import { CaretDownIcon } from "@phosphor-icons/react/dist/csr/CaretDown";
import { materialTypeLabels } from "../../../utils/materialLabels";
import type {
  CatalogFacets,
  CatalogFilters as Filters,
  MaterialType,
} from "../../../types/material";
import { Checkbox } from "../../common/Checkbox/Checkbox";
import { Button } from "../../common/Button/Button";
import { SearchInput } from "../../common/SearchInput/SearchInput";
import { FilterGroup } from "./FilterGroup";
import styles from "./CatalogFilters.module.css";

interface CatalogFiltersProps {
  value: Filters;
  facets: CatalogFacets;
  onChange: (filters: Filters) => void;
  onReset: () => void;
}
const visibleTypes: MaterialType[] = ["lecture", "book", "article", "video"];
const toggle = <T,>(items: T[], value: T) =>
  items.includes(value)
    ? items.filter((item) => item !== value)
    : [...items, value];

export function CatalogFilters({
  value,
  facets,
  onChange,
  onReset,
}: CatalogFiltersProps) {
  const { disciplines, teachers } = facets;
  const [allDisciplines, setAllDisciplines] = useState(false);
  const [allTeachers, setAllTeachers] = useState(false);
  const [teacherQuery, setTeacherQuery] = useState("");
  const shownDisciplines = allDisciplines
    ? disciplines
    : disciplines.slice(0, 3);
  const shownTeachers = (
    teacherQuery || allTeachers ? teachers : teachers.slice(0, 4)
  ).filter((teacher) =>
    teacher.name
      .toLocaleLowerCase("ru")
      .includes(teacherQuery.toLocaleLowerCase("ru")),
  );
  return (
    <section className={styles.filters} aria-label="Фильтры каталога">
      <div className={styles.heading}>
        <h2>Фильтры</h2>
        <Button onClick={onReset}>Сбросить все</Button>
      </div>
      <FilterGroup title="Тип материала">
        {visibleTypes.map((type) => (
          <Checkbox
            key={type}
            label={materialTypeLabels[type]}
            count={facets.types[type] ?? 0}
            checked={value.types.includes(type)}
            onChange={() =>
              onChange({ ...value, types: toggle(value.types, type) })
            }
          />
        ))}
      </FilterGroup>
      <FilterGroup title="Дисциплина">
        {shownDisciplines.map((discipline) => (
          <Checkbox
            key={discipline.id}
            label={discipline.name}
            count={discipline.materialCount}
            checked={value.disciplineIds.includes(discipline.id)}
            onChange={() =>
              onChange({
                ...value,
                disciplineIds: toggle(value.disciplineIds, discipline.id),
              })
            }
          />
        ))}
        <Button
          className={styles.more}
          onClick={() => setAllDisciplines((current) => !current)}
          aria-expanded={allDisciplines}
        >
          {allDisciplines ? "Скрыть дисциплины" : "Показать все дисциплины"}
          <CaretDownIcon
            size={12}
            className={allDisciplines ? styles.rotated : ""}
            aria-hidden="true"
          />
        </Button>
      </FilterGroup>
      <FilterGroup title="Преподаватель">
        <div className={styles.teacherSearch}>
          <SearchInput
            compact
            value={teacherQuery}
            onChange={setTeacherQuery}
            label="Поиск преподавателя"
            placeholder="Поиск преподавателя…"
          />
        </div>
        {shownTeachers.map((teacher) => (
          <Checkbox
            key={teacher.id}
            label={teacher.shortName}
            count={teacher.materialCount}
            checked={value.teacherIds.includes(teacher.id)}
            onChange={() =>
              onChange({
                ...value,
                teacherIds: toggle(value.teacherIds, teacher.id),
              })
            }
          />
        ))}
        {!shownTeachers.length && (
          <p className={styles.noTeachers}>Преподаватель не найден</p>
        )}
        {!teacherQuery && (
          <Button
            className={styles.more}
            onClick={() => setAllTeachers((current) => !current)}
            aria-expanded={allTeachers}
          >
            {allTeachers
              ? "Скрыть преподавателей"
              : "Показать всех преподавателей"}
            <CaretDownIcon
              size={12}
              className={allTeachers ? styles.rotated : ""}
              aria-hidden="true"
            />
          </Button>
        )}
      </FilterGroup>
      <FilterGroup title="Курс">
        {facets.courses.map(({ value: course, materialCount }) => (
          <Checkbox
            key={course}
            label={`${course} курс`}
            count={materialCount}
            checked={value.courses.includes(course)}
            onChange={() =>
              onChange({ ...value, courses: toggle(value.courses, course) })
            }
          />
        ))}
      </FilterGroup>
      <FilterGroup title="Год">
        {facets.years.map(({ value: year, materialCount }) => (
          <Checkbox
            key={year}
            label={String(year)}
            count={materialCount}
            checked={value.years.includes(year)}
            onChange={() =>
              onChange({ ...value, years: toggle(value.years, year) })
            }
          />
        ))}
      </FilterGroup>
    </section>
  );
}
