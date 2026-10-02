import type { MaterialSectionData } from "../../../types/material";
import { DisciplineMaterialItem } from "../DisciplineMaterialItem/DisciplineMaterialItem";
import styles from "./MaterialSection.module.css";

interface MaterialSectionProps {
  section: MaterialSectionData;
}

export function MaterialSection({ section }: MaterialSectionProps) {
  if (!section.items.length) {
    return null;
  }

  const headingId = `section-${section.id}`;
  const compactDescription = section.mobileDescription ?? section.description;

  return (
    <section
      className={styles.section}
      aria-labelledby={headingId}
    >
      <h2 id={headingId}>{section.title}</h2>
      <p className={styles.description}>
        <span className={styles.fullDescription}>{section.description}</span>
        <span className={styles.mobileDescription}>
          {compactDescription}
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
