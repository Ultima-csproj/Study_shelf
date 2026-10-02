import type { ReactNode } from "react";
import { Link, useLocation } from "react-router-dom";
import { BookBookmarkIcon } from "@phosphor-icons/react/dist/csr/BookBookmark";
import { ArrowRightIcon } from "@phosphor-icons/react/dist/csr/ArrowRight";
import type { CollectionSummary } from "../../../types/collection";
import {
  collectionMaterialCount,
  compilerNames,
} from "../../../utils/collectionLabels";
import styles from "./CollectionListItem.module.css";
import {
  collectionsForDiscipline,
  locationPath,
  withNavigationOrigin,
} from "../../../utils/navigation";

interface CollectionListItemProps {
  collection: CollectionSummary;
}

export function CollectionListItem({ collection }: CollectionListItemProps) {
  const location = useLocation();
  const path = withNavigationOrigin(
    `/collections/${collection.slug}`,
    locationPath(location),
  );
  const names = compilerNames(collection.contributors);
  const count = collectionMaterialCount(collection.materialCount);
  let courseLabel: string | null = null;

  if (collection.course !== null) {
    courseLabel = `${collection.course} курс`;
  }

  const metadataParts: string[] = [];

  if (courseLabel) {
    metadataParts.push(courseLabel);
  }

  if (collection.estimatedTime) {
    metadataParts.push(collection.estimatedTime);
  }

  const metadata = metadataParts.join(" · ");
  let disciplinePath: string | null = null;

  if (collection.discipline) {
    disciplinePath = collectionsForDiscipline(collection.discipline.id);
  }

  let disciplineLink: ReactNode = null;

  if (disciplinePath && collection.discipline) {
    disciplineLink = (
      <>
        {" · "}
        <Link to={disciplinePath} title="Подборки по этой дисциплине">
          {collection.discipline.name}
        </Link>
      </>
    );
  }

  let metadataText = "";

  if (metadata) {
    metadataText = ` · ${metadata}`;
  }

  return (
    <li className={styles.row}>
      <Link
        to={path}
        className={styles.preview}
        tabIndex={-1}
        aria-hidden="true"
      >
        <BookBookmarkIcon size={38} aria-hidden="true" />
      </Link>
      <article className={styles.body}>
        <h2>
          <Link to={path}>{collection.title}</Link>
        </h2>
        <p className={styles.description}>{collection.shortDescription}</p>
        <p className={styles.contributors}>Составили: {names}</p>
        <p className={styles.metadata}>
          {count}
          {disciplineLink}
          {metadataText}
        </p>
      </article>
      <Link
        to={path}
        className={styles.open}
        aria-label={`Открыть подборку: ${collection.title}`}
      >
        Открыть
        <ArrowRightIcon size={20} aria-hidden="true" />
      </Link>
    </li>
  );
}
