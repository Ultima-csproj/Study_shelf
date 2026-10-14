import { Link, useLocation } from "react-router-dom";
import type { ReactNode } from "react";
import type { CourseMaterial } from "../../../types/material";
import { MaterialPreview } from "../../common/MaterialPreview/MaterialPreview";
import { Badge } from "../../common/Badge/Badge";
import { SaveAction } from "../../common/SaveAction/SaveAction";
import styles from "./DisciplineMaterialItem.module.css";
import {
  locationPath,
  materialPath,
  withNavigationOrigin,
} from "../../../utils/navigation";
const date = new Intl.DateTimeFormat("ru-RU", {
  day: "numeric",
  month: "short",
  year: "numeric",
  timeZone: "UTC",
});

interface DisciplineMaterialItemProps {
  item: CourseMaterial;
}

export function DisciplineMaterialItem({
  item,
}: DisciplineMaterialItemProps) {
  const location = useLocation();
  const path = withNavigationOrigin(
    materialPath(item.material.id, item.material.source),
    locationPath(location),
  );
  let duration: string | null = null;

  if (item.material.durationSeconds !== null) {
    duration = `${Math.ceil(item.material.durationSeconds / 60)} мин`;
  }

  const typeLabels = { book: "Учебник", lab: "Практика" };
  let mobileType = item.formatLabel;

  if (item.previewKind) {
    mobileType = "PDF";
  } else if (item.material.type === "book" || item.material.type === "lab") {
    mobileType = typeLabels[item.material.type];
  }

  let typeBadge: ReactNode;

  if (item.material.type === "video") {
    typeBadge = <Badge type="video" />;
  } else {
    typeBadge = <span className={styles.mobileBadge}>{mobileType}</span>;
  }

  const addedAt = date.format(new Date(item.addedAt)).replace(/\s?г\.$/, "");
  let formatMetadata: ReactNode = null;

  if (item.material.type !== "video") {
    formatMetadata = (
      <>
        <span className={styles.format}>{item.formatLabel}</span>
        <span className={styles.dot} aria-hidden="true">
          ·
        </span>
      </>
    );
  }

  let publicationMetadata: ReactNode;

  if (item.publicationYear) {
    publicationMetadata = <span>{item.publicationYear}</span>;
  } else {
    publicationMetadata = <time dateTime={item.addedAt}>{addedAt}</time>;
  }

  let additionalMetadata: ReactNode = null;
  const hasAdditionalMetadata = Boolean(item.fileSize || duration);

  if (hasAdditionalMetadata) {
    const sizeOrDuration = item.fileSize ?? duration;

    additionalMetadata = (
      <>
        <span aria-hidden="true">·</span>
        <span>{sizeOrDuration}</span>
      </>
    );
  }

  const compactDescription = item.summary ?? item.description;

  return (
    <li className={styles.item}>
      <span className={styles.number} aria-label={`Материал ${item.number}`}>
        {item.number}
      </span>
      <Link
        to={path}
        className={styles.preview}
        aria-label={`Открыть материал: ${item.title}`}
      >
        <MaterialPreview
          material={item.material}
          variant="course"
          kind={item.previewKind}
        />
      </Link>
      <div className={styles.type}>{typeBadge}</div>
      <article className={styles.body}>
        <h3>
          <Link to={path}>{item.title}</Link>
        </h3>
        <p className={styles.meta}>
          {formatMetadata}
          <span>{item.contributor}</span>
          <span aria-hidden="true">·</span>
          {publicationMetadata}
          {additionalMetadata}
        </p>
        <p className={styles.description}>
          <span className={styles.fullDescription}>{item.description}</span>
          <span className={styles.compactDescription}>
            {compactDescription}
          </span>
        </p>
      </article>
      <div className={styles.save}>
        <SaveAction id={item.material.id} title={item.title} />
      </div>
    </li>
  );
}
