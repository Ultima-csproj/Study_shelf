import { useCallback, type ReactNode } from "react";
import { Link, useLocation, useSearchParams } from "react-router-dom";
import { getLibrary } from "../../api/library";
import { useApiResource } from "../../hooks/useApiResource";
import { useViewer } from "../../hooks/useViewer";
import { CollectionListItem } from "../../components/collections/CollectionListItem/CollectionListItem";
import { SaveAction } from "../../components/common/SaveAction/SaveAction";
import styles from "./LibraryPage.module.css";
import {
  locationPath,
  materialPath,
  withNavigationOrigin,
} from "../../utils/navigation";

export function LibraryPage() {
  const { viewer, savedMaterials, savedCollectionIds } = useViewer();
  const [params, setParams] = useSearchParams();
  const location = useLocation();
  const parentPath = locationPath(location);
  let tab: "saved" | "collections" = "saved";

  if (params.get("tab") === "collections") {
    tab = "collections";
  }

  let userId = 0;

  if (viewer.kind === "authenticated") {
    userId = viewer.user.id;
  }

  const load = useCallback(
    (signal: AbortSignal) => {
      return getLibrary(userId, savedMaterials, savedCollectionIds, signal);
    },
    [userId, savedMaterials, savedCollectionIds],
  );
  const { result, error, loading, retry } = useApiResource(
    load,
    "Не удалось загрузить библиотеку",
  );
  const materials = result?.materials ?? [];
  const savedCollections = result?.savedCollections ?? [];
  const contributedCollections = result?.contributedCollections ?? [];

  const changeTab = (next: "saved" | "collections") => {
    setParams({ tab: next }, { replace: true, flushSync: true });
  };

  const handleSavedTabClick = () => {
    changeTab("saved");
  };

  const handleCollectionsTabClick = () => {
    changeTab("collections");
  };

  let content: ReactNode;

  if (loading) {
    content = <p role="status">Загружаем библиотеку…</p>;
  } else if (error) {
    content = (
      <div>
        <p role="alert">{error}</p>
        <button type="button" onClick={retry}>
          Повторить загрузку
        </button>
      </div>
    );
  } else if (tab === "saved") {
    let emptyMaterialsMessage: ReactNode = null;

    if (materials.length === 0) {
      emptyMaterialsMessage = (
        <div className={styles.empty}>
          <p>Здесь появятся материалы, которые вы сохраните.</p>
          <Link to="/catalog">Найти материалы в каталоге →</Link>
        </div>
      );
    }

    content = (
      <section aria-labelledby="saved-materials-heading">
        <h2 id="saved-materials-heading">Сохранённые материалы</h2>
        {emptyMaterialsMessage}
        <ul className={styles.materials}>
          {materials.map((material) => {
            const path = withNavigationOrigin(
              materialPath(material.id, material.source),
              parentPath,
            );

            return (
              <li key={`${material.source}:${material.id}`}>
                <div>
                  <h3>
                    <Link to={path}>{material.title}</Link>
                  </h3>
                  <p>{material.placements.map((placement) => placement.discipline.name).join(" · ")}</p>
                </div>
                <SaveAction
                  id={material.id}
                  title={material.title}
                  source={material.source}
                />
              </li>
            );
          })}
        </ul>
      </section>
    );
  } else {
    let emptyCollectionsMessage: ReactNode = null;

    if (savedCollections.length === 0) {
      emptyCollectionsMessage = (
        <div className={styles.empty}>
          <p>Вы пока не сохранили ни одной подборки.</p>
          <Link to="/collections">Посмотреть публичные подборки →</Link>
        </div>
      );
    }

    let contributionsSection: ReactNode = null;

    if (contributedCollections.length > 0) {
      contributionsSection = (
        <section
          className={styles.contributions}
          aria-labelledby="contributed-collections-heading"
        >
          <h2 id="contributed-collections-heading">
            Подборки с вашим участием
          </h2>
          <ul className={styles.collections}>
            {contributedCollections.map((collection) => (
              <CollectionListItem
                key={collection.id}
                collection={collection}
              />
            ))}
          </ul>
        </section>
      );
    }

    content = (
      <>
        <section aria-labelledby="saved-collections-heading">
          <h2 id="saved-collections-heading">Сохранённые подборки</h2>
          {emptyCollectionsMessage}
          <ul className={styles.collections}>
            {savedCollections.map((collection) => (
              <CollectionListItem key={collection.id} collection={collection} />
            ))}
          </ul>
        </section>
        {contributionsSection}
      </>
    );
  }

  let savedTabCurrent: "page" | undefined = undefined;
  let collectionsTabCurrent: "page" | undefined = undefined;

  if (tab === "saved") {
    savedTabCurrent = "page";
  }

  if (tab === "collections") {
    collectionsTabCurrent = "page";
  }

  return (
    <div className={styles.page}>
      <h1>Моя библиотека</h1>
      <p className={styles.subtitle}>
        Сохранённые материалы и учебные маршруты.
      </p>
      <nav className={styles.tabs} aria-label="Разделы библиотеки">
        <button
          type="button"
          aria-current={savedTabCurrent}
          onClick={handleSavedTabClick}
        >
          Сохранённое ({savedMaterials.length})
        </button>
        <button
          type="button"
          aria-current={collectionsTabCurrent}
          onClick={handleCollectionsTabClick}
        >
          Подборки ({savedCollectionIds.length})
        </button>
      </nav>
      <div className={styles.content}>{content}</div>
    </div>
  );
}
