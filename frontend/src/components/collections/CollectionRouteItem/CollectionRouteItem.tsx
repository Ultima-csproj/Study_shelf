import { Link, useLocation } from "react-router-dom";
import { LinkIcon } from "@phosphor-icons/react/dist/csr/Link";
import type { ReactNode } from "react";
import { MaterialPreview } from "../../common/MaterialPreview/MaterialPreview";
import type { CollectionItemView } from "../../../types/collection";
import type { PreviewKind } from "../../../types/material";
import styles from "./CollectionRouteItem.module.css";
import { materialPath, withNavigationOrigin } from "../../../utils/navigation";

interface CollectionRouteItemProps {
  item: CollectionItemView;
  sectionNumber: number;
  sectionId: string;
}

export function CollectionRouteItem({
  item,
  sectionNumber,
  sectionId,
}: CollectionRouteItemProps) {
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
    if (sourceDocument) {
      previewKind = "document";
    } else {
      previewKind = "pdf";
    }
  }

  let durationLabel: string | null = null;

  if (material.durationSeconds !== null) {
    const minutes = Math.ceil(material.durationSeconds / 60);
    durationLabel = `${minutes} мин`;
  }

  let lastLabel: string | null = "PDF";

  if (material.type === "video") {
    lastLabel = durationLabel;
  } else if (externalArticle) {
    lastLabel = null;
  }

  const metadataParts = [item.formatLabel, item.attributionLabel];

  if (lastLabel) {
    metadataParts.push(lastLabel);
  }

  const metadata = metadataParts.join(" · ");
  let previewContent: ReactNode;

  if (externalArticle) {
    previewContent = (
      <span className={styles.external}>
        <LinkIcon size={31} aria-hidden="true" />
      </span>
    );
  } else {
    previewContent = (
      <MaterialPreview
        material={material}
        variant="collection"
        kind={previewKind}
      />
    );
  }

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
        {previewContent}
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
