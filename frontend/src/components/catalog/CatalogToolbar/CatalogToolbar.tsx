import type { ReactNode } from "react";
import { XIcon } from "@phosphor-icons/react/dist/csr/X";
import { FunnelIcon } from "@phosphor-icons/react/dist/csr/Funnel";
import { materialTypeLabels } from "../../../utils/materialLabels";
import type {
  CatalogFacets,
  CatalogFilters,
  CatalogSort,
} from "../../../types/material";
import { Button } from "../../common/Button/Button";
import { Select } from "../../common/Select/Select";
import styles from "./CatalogToolbar.module.css";

interface FilterChip {
  key: string;
  label: string;
  remove: () => void;
}

interface CatalogToolbarProps {
  filters: CatalogFilters;
  facets: CatalogFacets;
  sort: CatalogSort;
  onFiltersChange: (filters: CatalogFilters) => void;
  onSortChange: (sort: CatalogSort) => void;
  onReset: () => void;
  onOpenFilters: () => void;
}
export function CatalogToolbar({
  filters,
  facets,
  sort,
  onFiltersChange,
  onSortChange,
  onReset,
  onOpenFilters,
}: CatalogToolbarProps) {
  const { disciplines, teachers } = facets;
  const chips: FilterChip[] = [];

  for (const type of filters.types) {
    const chip: FilterChip = {
      key: `type-${type}`,
      label: materialTypeLabels[type],
      remove: () => {
        const types = filters.types.filter((item) => item !== type);
        const nextFilters = { ...filters, types };
        onFiltersChange(nextFilters);
      },
    };

    chips.push(chip);
  }

  for (const id of filters.disciplineIds) {
    const discipline = disciplines.find((item) => item.id === id);
    const label = discipline?.name ?? String(id);
    const chip: FilterChip = {
      key: `discipline-${id}`,
      label,
      remove: () => {
        const disciplineIds = filters.disciplineIds.filter(
          (item) => item !== id,
        );
        const nextFilters = { ...filters, disciplineIds };
        onFiltersChange(nextFilters);
      },
    };

    chips.push(chip);
  }

  for (const id of filters.teacherIds) {
    const teacher = teachers.find((item) => item.id === id);
    const label = teacher?.shortName ?? String(id);
    const chip: FilterChip = {
      key: `teacher-${id}`,
      label,
      remove: () => {
        const teacherIds = filters.teacherIds.filter((item) => item !== id);
        const nextFilters = { ...filters, teacherIds };
        onFiltersChange(nextFilters);
      },
    };

    chips.push(chip);
  }

  for (const course of filters.courses) {
    const chip: FilterChip = {
      key: `course-${course}`,
      label: `${course} курс`,
      remove: () => {
        const courses = filters.courses.filter((item) => item !== course);
        const nextFilters = { ...filters, courses };
        onFiltersChange(nextFilters);
      },
    };

    chips.push(chip);
  }

  for (const year of filters.years) {
    const chip: FilterChip = {
      key: `year-${year}`,
      label: String(year),
      remove: () => {
        const years = filters.years.filter((item) => item !== year);
        const nextFilters = { ...filters, years };
        onFiltersChange(nextFilters);
      },
    };

    chips.push(chip);
  }

  for (const tag of filters.tags) {
    const chip: FilterChip = {
      key: `tag-${tag}`,
      label: tag,
      remove: () => {
        const tags = filters.tags.filter((item) => item !== tag);
        const nextFilters = { ...filters, tags };
        onFiltersChange(nextFilters);
      },
    };

    chips.push(chip);
  }

  const hasActiveFilters = chips.length > 0 || Boolean(filters.query);
  let filterButtonLabel = "Фильтры";

  if (chips.length > 0) {
    filterButtonLabel = `Фильтры (${chips.length})`;
  }

  const sortOptions = [
    { value: "newest", label: "Сначала новые" },
    { value: "oldest", label: "Сначала старые" },
    { value: "title", label: "По названию" },
  ];

  const handleSortChange = (value: string) => {
    const nextSort = value as CatalogSort;
    onSortChange(nextSort);
  };

  let resetButton: ReactNode = null;

  if (hasActiveFilters) {
    resetButton = (
      <Button onClick={onReset} className={styles.reset}>
        Сбросить фильтры
      </Button>
    );
  }

  return (
    <div className={styles.toolbar}>
      <Button
        variant="secondary"
        className={styles.mobileFilters}
        onClick={onOpenFilters}
      >
        <FunnelIcon size={16} aria-hidden="true" />
        {filterButtonLabel}
      </Button>
      <div className={styles.chips} aria-label="Выбранные фильтры">
        {chips.map((chip) => (
          <button
            type="button"
            key={chip.key}
            onClick={chip.remove}
            className={styles.chip}
            aria-label={`Убрать фильтр: ${chip.label}`}
          >
            {chip.label}
            <XIcon size={13} aria-hidden="true" />
          </button>
        ))}
        {resetButton}
      </div>
      <div className={styles.sort}>
        <span aria-hidden="true">Сортировка:</span>
        <Select
          label="Сортировка материалов"
          value={sort}
          onChange={handleSortChange}
          options={sortOptions}
        />
      </div>
    </div>
  );
}
