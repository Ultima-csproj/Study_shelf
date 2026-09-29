import type { CatalogMaterial } from "../../../types/material";
import { MaterialListItem } from "../MaterialListItem/MaterialListItem";
import { Button } from "../../common/Button/Button";
import styles from "./MaterialList.module.css";

interface MaterialListProps {
  materials: CatalogMaterial[];
  loading: boolean;
  error: string | null;
  selectedTags: string[];
  onTagClick: (tag: string) => void;
  onReset: () => void;
  onRetry: () => void;
}
export function MaterialList({
  materials,
  loading,
  error,
  selectedTags,
  onTagClick,
  onReset,
  onRetry,
}: MaterialListProps) {
  if (error)
    return (
      <div role="alert" className={styles.state}>
        <h2>Не удалось загрузить каталог</h2>
        <p>{error}</p>
        <Button variant="secondary" onClick={onRetry}>
          Повторить загрузку
        </Button>
      </div>
    );
  if (!loading && !materials.length)
    return (
      <div className={styles.state} role="status">
        <h2>Материалы не найдены</h2>
        <p>Попробуйте другой запрос или сбросьте фильтры.</p>
        <Button variant="secondary" onClick={onReset}>
          Сбросить фильтры
        </Button>
      </div>
    );
  return (
    <ul
      className={styles.list}
      aria-label="Найденные материалы"
      aria-busy={loading}
    >
      {materials.map((material) => (
        <MaterialListItem
          key={material.id}
          material={material}
          selectedTags={selectedTags}
          onTagClick={onTagClick}
        />
      ))}
      {loading && !materials.length && (
        <li className={styles.state} role="status">
          Загрузка материалов…
        </li>
      )}
    </ul>
  );
}
