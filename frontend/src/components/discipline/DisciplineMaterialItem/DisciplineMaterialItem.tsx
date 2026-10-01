import { Link, useLocation } from "react-router-dom";
import type { CourseMaterial } from "../../../types/material";
import { MaterialPreview } from "../../common/MaterialPreview/MaterialPreview";
import { Badge } from "../../common/Badge/Badge";
import { SaveAction } from "../../common/SaveAction/SaveAction";
import styles from "./DisciplineMaterialItem.module.css";
import { durationMinutes } from "../../../utils/format";
import { locationPath, withNavigationOrigin } from "../../../utils/navigation";
const date = new Intl.DateTimeFormat("ru-RU", {
  day: "numeric",
  month: "short",
  year: "numeric",
  timeZone: "UTC",
});
export function DisciplineMaterialItem({ item }: { item: CourseMaterial }) {
  const location = useLocation();
  const path = withNavigationOrigin(
    `/materials/${item.material.id}`,
    locationPath(location),
  );
  const duration = item.material.duration
    ? durationMinutes(item.material.duration)
    : null;
  const typeLabels = { book: "Учебник", lab: "Практика" };
  let mobileType = item.formatLabel;
  if (item.previewKind) {
    mobileType = "PDF";
  } else if (item.material.type === "book" || item.material.type === "lab") {
    mobileType = typeLabels[item.material.type];
  }
  const typeBadge =
    item.material.type === "video" ? (
      <Badge type="video" />
    ) : (
      <span className={styles.mobileBadge}>{mobileType}</span>
    );
  const addedAt = date.format(new Date(item.addedAt)).replace(/\s?г\.$/, "");
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
          {item.material.type !== "video" && (
            <>
              <span className={styles.format}>{item.formatLabel}</span>
              <span className={styles.dot} aria-hidden="true">
                ·
              </span>
            </>
          )}
          <span>{item.contributor}</span>
          <span aria-hidden="true">·</span>
          {item.publicationYear ? (
            <span>{item.publicationYear}</span>
          ) : (
            <time dateTime={item.addedAt}>{addedAt}</time>
          )}
          {(item.fileSize || duration) && (
            <>
              <span aria-hidden="true">·</span>
              <span>{item.fileSize ?? duration}</span>
            </>
          )}
        </p>
        <p className={styles.description}>
          <span className={styles.fullDescription}>{item.description}</span>
          <span className={styles.compactDescription}>
            {item.summary ?? item.description}
          </span>
        </p>
      </article>
      <div className={styles.save}>
        <SaveAction id={item.material.id} title={item.title} />
      </div>
    </li>
  );
}
