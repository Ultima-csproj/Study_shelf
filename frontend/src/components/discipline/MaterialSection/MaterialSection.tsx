import type { MaterialSectionData } from "../../../types/material";
import { getDisciplineSectionItems } from "../../../api/disciplines";
import { useSectionItems } from "../../../hooks/useSectionItems";
import { DisciplineMaterialItem } from "../DisciplineMaterialItem/DisciplineMaterialItem";
import { Button } from "../../common/Button/Button";
import styles from "./MaterialSection.module.css";

interface MaterialSectionProps {
  disciplineId: number;
  section: MaterialSectionData;
}
export function MaterialSection({ disciplineId, section }: MaterialSectionProps) {
  const load = (page: number, signal: AbortSignal) => {
    return getDisciplineSectionItems(disciplineId, section.id, page, section.initialPageSize, signal);
  };
  const { items, loading, error, hasMore, loadMore } = useSectionItems(section.items, section.stepCount, load);
  if (!items.length) {
    return null;
  }
  const headingId = `section-${section.id}`;
  const compactDescription = section.mobileDescription ?? section.description;
  return (
    <section className={styles.section} aria-labelledby={headingId}>
      <h2 id={headingId}>{section.title}</h2>
      <p className={styles.description}>
        <span className={styles.fullDescription}>{section.description}</span>
        <span className={styles.mobileDescription}>{compactDescription}</span>
      </p>
      <ol>
        {items.map((item) => (
          <DisciplineMaterialItem key={item.id} item={item} />
        ))}
      </ol>
      {error && <p role="alert">{error}</p>}
      {hasMore && (
        <Button variant="secondary" onClick={loadMore} disabled={loading}>
          {loading ? "Загрузка…" : "Показать ещё"}
        </Button>
      )}
    </section>
  );
}
