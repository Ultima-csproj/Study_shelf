import type { MaterialSectionData } from "../../../types/material";
import { DisciplineMaterialItem } from "../DisciplineMaterialItem/DisciplineMaterialItem";
import styles from "./MaterialSection.module.css";
export function MaterialSection({ section }: { section: MaterialSectionData }) {
  if (!section.items.length) {
    return null;
  }
  return (
    <section
      className={styles.section}
      aria-labelledby={`section-${section.id}`}
    >
      <h2 id={`section-${section.id}`}>{section.title}</h2>
      <p className={styles.description}>
        <span className={styles.fullDescription}>{section.description}</span>
        <span className={styles.mobileDescription}>
          {section.mobileDescription ?? section.description}
        </span>
      </p>
      <ol>
        {section.items.map((item) => (
          <DisciplineMaterialItem key={item.material.id} item={item} />
        ))}
      </ol>
    </section>
  );
}
