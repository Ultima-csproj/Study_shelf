import { UserIcon } from "@phosphor-icons/react/dist/csr/User";
import { Link, useLocation } from "react-router-dom";
import type { ReactNode } from "react";
import type { CatalogMaterial } from "../../../types/material";
import { Badge } from "../../common/Badge/Badge";
import { MaterialPreview } from "../../common/MaterialPreview/MaterialPreview";
import { authorLabel, exactPublication, formatDuration, publicationLabel, relativePublication } from "../../../utils/materialLabels";
import styles from "./MaterialListItem.module.css";
import { locationPath, materialPath, withNavigationOrigin } from "../../../utils/navigation";

interface MaterialListItemProps {
  material: CatalogMaterial;
  selectedTags: string[];
  onTagClick: (tag: string) => void;
}
export function MaterialListItem({ material, selectedTags, onTagClick }: MaterialListItemProps) {
  const location = useLocation();
  const path = withNavigationOrigin(
    materialPath(material.id, material.source),
    locationPath(location),
  );
  const authors = authorLabel(material.credits);
  const dateLabel = publicationLabel(material.firstPublishedAt, material.publishedAt);
  const date = relativePublication(material.publishedAt);
  const exactDate = exactPublication(material.publishedAt);
  const duration = formatDuration(material.durationSeconds);
  let durationContent: ReactNode = null;
  if (duration) {
    durationContent = <span>{duration}</span>;
  }
  const placementLinks = material.placements.map((placement) => {
    const discipline = placement.discipline;
    return (
      <Link
        key={discipline.id}
        className={styles.discipline}
        to={"/disciplines/" + discipline.slug}
      >
        {discipline.name}
      </Link>
    );
  });
  let educationContent: ReactNode = placementLinks;
  if (placementLinks.length === 0) {
    educationContent = <span>Самостоятельный материал</span>;
  }
  return (
    <li className={styles.item}>
      <Link to={path} aria-label={"Открыть материал: " + material.title}>
        <MaterialPreview material={material} />
      </Link>
      <article className={styles.body}>
        <div className={styles.titleRow}>
          <Badge type={material.type} />
          <h2><Link to={path}>{material.title}</Link></h2>
          <time dateTime={material.publishedAt} title={exactDate} className={styles.date}>
            {dateLabel} {date}
          </time>
        </div>
        <div className={styles.meta}>
          <span title={authors}>
            <UserIcon size={15} aria-hidden="true" />
            {authors}
          </span>
          {educationContent}
          {durationContent}
        </div>
        <p className={styles.description}>{material.description}</p>
        <div className={styles.tags} aria-label="Теги материала">
          {material.tags.map((tag) => (
            <button
              type="button"
              key={tag}
              className={styles.tag}
              aria-pressed={selectedTags.includes(tag)}
              onClick={() => onTagClick(tag)}
              title={"Фильтровать по тегу «" + tag + "»"}
            >
              {tag}
            </button>
          ))}
        </div>
      </article>
    </li>
  );
}
