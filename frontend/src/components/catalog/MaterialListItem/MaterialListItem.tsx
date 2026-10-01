import { UserIcon } from "@phosphor-icons/react/dist/csr/User";
import { Link, useLocation } from "react-router-dom";
import type { CatalogMaterial } from "../../../types/material";
import { Badge } from "../../common/Badge/Badge";
import { MaterialPreview } from "../../common/MaterialPreview/MaterialPreview";
import styles from "./MaterialListItem.module.css";
import {
  locationPath,
  materialPath,
  withNavigationOrigin,
} from "../../../utils/navigation";

const dateFormatter = new Intl.DateTimeFormat("ru-RU", {
  day: "numeric",
  month: "long",
  year: "numeric",
  timeZone: "UTC",
});
interface MaterialListItemProps {
  material: CatalogMaterial;
  selectedTags: string[];
  onTagClick: (tag: string) => void;
}
export function MaterialListItem({
  material,
  selectedTags,
  onTagClick,
}: MaterialListItemProps) {
  const { teacher, discipline } = material;
  const location = useLocation();
  const path = withNavigationOrigin(
    materialPath(material.id, material.source),
    locationPath(location),
  );
  return (
    <li className={styles.item}>
      <Link to={path} aria-label={`Открыть материал: ${material.title}`}>
        <MaterialPreview material={material} />
      </Link>
      <article className={styles.body}>
        <div className={styles.titleRow}>
          <Badge type={material.type} />
          <h2>
            <Link to={path}>{material.title}</Link>
          </h2>
          <time dateTime={material.createdAt} className={styles.date}>
            {material.dateLabel}{" "}
            {dateFormatter
              .format(new Date(material.createdAt))
              .replace(/\s?г\.$/, "")}
          </time>
        </div>
        <div className={styles.meta}>
          <span title={teacher?.name}>
            <UserIcon size={15} aria-hidden="true" />
            {teacher?.shortName}
          </span>
          <Link
            className={styles.discipline}
            to={`/disciplines/${discipline?.slug ?? material.disciplineId}`}
          >
            {discipline?.name}
          </Link>
          {!material.preview && material.duration && (
            <span>{material.duration}</span>
          )}
          {material.preview && material.duration && (
            <span className={styles.mobileDuration}>{material.duration}</span>
          )}
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
              title={`Фильтровать по тегу «${tag}»`}
            >
              {tag}
            </button>
          ))}
        </div>
      </article>
    </li>
  );
}
