import { XIcon } from '@phosphor-icons/react/dist/csr/X'
import { FunnelIcon } from '@phosphor-icons/react/dist/csr/Funnel'
import { materialTypeLabels } from '../../../utils/materialLabels'
import type {
  CatalogFacets,
  CatalogFilters,
  CatalogSort,
} from '../../../types/material'
import { Button } from '../../common/Button/Button'
import { Select } from '../../common/Select/Select'
import styles from './CatalogToolbar.module.css'

interface CatalogToolbarProps {
  filters: CatalogFilters
  facets: CatalogFacets
  sort: CatalogSort
  onFiltersChange: (filters: CatalogFilters) => void
  onSortChange: (sort: CatalogSort) => void
  onReset: () => void
  onOpenFilters: () => void
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
  const { disciplines, teachers } = facets
  const chips = [
    ...filters.types.map((type) => ({
      key: `type-${type}`,
      label: materialTypeLabels[type],
      remove: () =>
        onFiltersChange({
          ...filters,
          types: filters.types.filter((item) => item !== type),
        }),
    })),
    ...filters.disciplineIds.map((id) => ({
      key: `discipline-${id}`,
      label: disciplines.find((item) => item.id === id)?.name ?? String(id),
      remove: () =>
        onFiltersChange({
          ...filters,
          disciplineIds: filters.disciplineIds.filter((item) => item !== id),
        }),
    })),
    ...filters.teacherIds.map((id) => ({
      key: `teacher-${id}`,
      label: teachers.find((item) => item.id === id)?.shortName ?? String(id),
      remove: () =>
        onFiltersChange({
          ...filters,
          teacherIds: filters.teacherIds.filter((item) => item !== id),
        }),
    })),
    ...filters.courses.map((course) => ({
      key: `course-${course}`,
      label: `${course} курс`,
      remove: () =>
        onFiltersChange({
          ...filters,
          courses: filters.courses.filter((item) => item !== course),
        }),
    })),
    ...filters.years.map((year) => ({
      key: `year-${year}`,
      label: String(year),
      remove: () =>
        onFiltersChange({
          ...filters,
          years: filters.years.filter((item) => item !== year),
        }),
    })),
    ...filters.tags.map((tag) => ({
      key: `tag-${tag}`,
      label: tag,
      remove: () =>
        onFiltersChange({
          ...filters,
          tags: filters.tags.filter((item) => item !== tag),
        }),
    })),
  ]
  return (
    <div className={styles.toolbar}>
      <Button
        variant="secondary"
        className={styles.mobileFilters}
        onClick={onOpenFilters}
      >
        <FunnelIcon
          size={16}
          aria-hidden="true"
        />
        Фильтры{chips.length ? ` (${chips.length})` : ''}
      </Button>
      <div
        className={styles.chips}
        aria-label="Выбранные фильтры"
      >
        {chips.map((chip) => (
          <button
            type="button"
            key={chip.key}
            onClick={chip.remove}
            className={styles.chip}
            aria-label={`Убрать фильтр: ${chip.label}`}
          >
            {chip.label}
            <XIcon
              size={13}
              aria-hidden="true"
            />
          </button>
        ))}
        {(chips.length > 0 || filters.query) && (
          <Button
            onClick={onReset}
            className={styles.reset}
          >
            Сбросить фильтры
          </Button>
        )}
      </div>
      <div className={styles.sort}>
        <span aria-hidden="true">Сортировка:</span>
        <Select
          label="Сортировка материалов"
          value={sort}
          onChange={(value) => onSortChange(value as CatalogSort)}
          options={[
            { value: 'newest', label: 'Сначала новые' },
            { value: 'oldest', label: 'Сначала старые' },
            { value: 'title', label: 'По названию' },
          ]}
        />
      </div>
    </div>
  )
}
