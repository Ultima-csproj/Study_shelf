import { useState, type ReactNode } from "react";
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
  filters: Filters;
  facets: CatalogFacets;
  onChange: (filters: Filters) => void;
  onReset: () => void;
}
const visibleTypes: MaterialType[] = ["lecture", "book", "article", "video"];

function toggle<T>(items: T[], value: T): T[] {
  const alreadySelected = items.includes(value);

  if (alreadySelected) {
    return items.filter((item) => item !== value);
  }

  return [...items, value];
}

export function CatalogFilters({
  filters,
  facets,
  onChange,
  onReset,
}: CatalogFiltersProps) {
  const { disciplines, teachers } = facets;
  const [allDisciplines, setAllDisciplines] = useState(false);
  const [allTeachers, setAllTeachers] = useState(false);
  const [teacherQuery, setTeacherQuery] = useState("");
  let shownDisciplines = disciplines.slice(0, 3);

  if (allDisciplines) {
    shownDisciplines = disciplines;
  }

  let teachersToFilter = teachers;

  if (!teacherQuery && !allTeachers) {
    teachersToFilter = teachers.slice(0, 4);
  }

  const normalizedTeacherQuery = teacherQuery.toLocaleLowerCase("ru");
  const shownTeachers = teachersToFilter.filter((teacher) => {
    const normalizedName = teacher.name.toLocaleLowerCase("ru");
    return normalizedName.includes(normalizedTeacherQuery);
  });

  const changeTypes = (type: MaterialType) => {
    const types = toggle(filters.types, type);
    const nextFilters = { ...filters, types };
    onChange(nextFilters);
  };

  const changeDisciplines = (disciplineId: number) => {
    const disciplineIds = toggle(filters.disciplineIds, disciplineId);
    const nextFilters = { ...filters, disciplineIds };
    onChange(nextFilters);
  };

  const changeTeachers = (teacherId: number) => {
    const teacherIds = toggle(filters.teacherIds, teacherId);
    const nextFilters = { ...filters, teacherIds };
    onChange(nextFilters);
  };

  const changeCourses = (course: number) => {
    const courses = toggle(filters.courses, course);
    const nextFilters = { ...filters, courses };
    onChange(nextFilters);
  };

  const changeYears = (year: number) => {
    const years = toggle(filters.years, year);
    const nextFilters = { ...filters, years };
    onChange(nextFilters);
  };

  const toggleAllDisciplines = () => {
    setAllDisciplines((currentlyShowingAll) => {
      if (currentlyShowingAll) {
        return false;
      }

      return true;
    });
  };

  const toggleAllTeachers = () => {
    setAllTeachers((currentlyShowingAll) => {
      if (currentlyShowingAll) {
        return false;
      }

      return true;
    });
  };

  const handleTeacherQueryChange = (query: string) => {
    setTeacherQuery(query);
  };

  let noTeachersMessage: ReactNode = null;

  if (shownTeachers.length === 0) {
    noTeachersMessage = (
      <p className={styles.noTeachers}>Преподаватель не найден</p>
    );
  }

  let disciplineToggleLabel = "Показать все дисциплины";
  let disciplineToggleClassName = "";

  if (allDisciplines) {
    disciplineToggleLabel = "Скрыть дисциплины";
    disciplineToggleClassName = styles.rotated;
  }

  let teacherToggleLabel = "Показать всех преподавателей";
  let teacherToggleClassName = "";

  if (allTeachers) {
    teacherToggleLabel = "Скрыть преподавателей";
    teacherToggleClassName = styles.rotated;
  }

  let teacherToggleButton: ReactNode = null;

  if (!teacherQuery) {
    teacherToggleButton = (
      <Button
        className={styles.more}
        onClick={toggleAllTeachers}
        aria-expanded={allTeachers}
      >
        {teacherToggleLabel}
        <CaretDownIcon
          size={12}
          className={teacherToggleClassName}
          aria-hidden="true"
        />
      </Button>
    );
  }

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
            checked={filters.types.includes(type)}
            onChange={() => changeTypes(type)}
          />
        ))}
      </FilterGroup>
      <FilterGroup title="Дисциплина">
        {shownDisciplines.map((discipline) => (
          <Checkbox
            key={discipline.id}
            label={discipline.name}
            count={discipline.materialCount}
            checked={filters.disciplineIds.includes(discipline.id)}
            onChange={() => changeDisciplines(discipline.id)}
          />
        ))}
        <Button
          className={styles.more}
          onClick={toggleAllDisciplines}
          aria-expanded={allDisciplines}
        >
          {disciplineToggleLabel}
          <CaretDownIcon
            size={12}
            className={disciplineToggleClassName}
            aria-hidden="true"
          />
        </Button>
      </FilterGroup>
      <FilterGroup title="Преподаватель">
        <div className={styles.teacherSearch}>
          <SearchInput
            compact
            value={teacherQuery}
            onChange={handleTeacherQueryChange}
            label="Поиск преподавателя"
            placeholder="Поиск преподавателя…"
          />
        </div>
        {shownTeachers.map((teacher) => (
          <Checkbox
            key={teacher.id}
            label={teacher.shortName}
            count={teacher.materialCount}
            checked={filters.teacherIds.includes(teacher.id)}
            onChange={() => changeTeachers(teacher.id)}
          />
        ))}
        {noTeachersMessage}
        {teacherToggleButton}
      </FilterGroup>
      <FilterGroup title="Курс">
        {facets.courses.map(({ value: course, materialCount }) => (
          <Checkbox
            key={course}
            label={`${course} курс`}
            count={materialCount}
            checked={filters.courses.includes(course)}
            onChange={() => changeCourses(course)}
          />
        ))}
      </FilterGroup>
      <FilterGroup title="Год">
        {facets.years.map(({ value: year, materialCount }) => (
          <Checkbox
            key={year}
            label={String(year)}
            count={materialCount}
            checked={filters.years.includes(year)}
            onChange={() => changeYears(year)}
          />
        ))}
      </FilterGroup>
    </section>
  );
}
