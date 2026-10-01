import { Link, useLocation } from "react-router-dom";
import { LinkIcon } from "@phosphor-icons/react/dist/csr/Link";
import { MaterialPreview } from "../../common/MaterialPreview/MaterialPreview";
import type { CollectionItemView } from "../../../types/collection";
import type { PreviewKind } from "../../../types/material";
import styles from "./CollectionRouteItem.module.css";
import { materialPath, withNavigationOrigin } from "../../../utils/navigation";

export function CollectionRouteItem({
  item,
  sectionNumber,
  sectionId,
}: {
  item: CollectionItemView;
  sectionNumber: number;
  sectionId: string;
}) {
  const material = item.material;
  const location = useLocation();
  const number = `${sectionNumber}.${item.position}`;
  const parentPath = `${location.pathname}${location.search}#${sectionId}`;
  const path = withNavigationOrigin(
    materialPath(material.id, material.source),
    parentPath,
  );
  const externalArticle = item.formatLabel === "Внешняя статья";
  const sourceDocument = item.attributionLabel.startsWith("Источник:");
  let previewKind: PreviewKind | undefined;

  if (material.type !== "video" && material.type !== "article") {
    previewKind = sourceDocument ? "document" : "pdf";
  }

  const duration = material.duration
    ? `${Number(material.duration.split(":")[0])} мин`
    : null;
  let lastLabel: string | null = "PDF";

  if (material.type === "video") {
    lastLabel = duration;
  } else if (externalArticle) {
    lastLabel = null;
  }

  const metadata = [item.formatLabel, item.attributionLabel, lastLabel]
    .filter(Boolean)
    .join(" · ");

  return (
    <li className={styles.row}>
      <span className={styles.number} aria-label={`Материал ${number}`}>
        {number}
      </span>
      <Link
        to={path}
        className={styles.preview}
        tabIndex={-1}
        aria-hidden="true"
      >
        {externalArticle ? (
          <span className={styles.external}>
            <LinkIcon size={31} aria-hidden="true" />
          </span>
        ) : (
          <MaterialPreview
            material={material}
            variant="collection"
            kind={previewKind}
          />
        )}
      </Link>
      <article className={styles.body}>
        <h3>
          <Link to={path}>{material.title}</Link>
        </h3>
        <p>{metadata}</p>
      </article>
    </li>
  );
}
