import { useCallback } from "react";
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
  const tab = params.get("tab") === "collections" ? "collections" : "saved";
  const userId = viewer.kind === "authenticated" ? viewer.user.id : 0;
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

  const changeTab = (next: string) => {
    setParams({ tab: next }, { replace: true, flushSync: true });
  };

  let content;

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
    content = (
      <section aria-labelledby="saved-materials-heading">
        <h2 id="saved-materials-heading">Сохранённые материалы</h2>
        {materials.length === 0 && (
          <div className={styles.empty}>
            <p>Здесь появятся материалы, которые вы сохраните.</p>
            <Link to="/catalog">Найти материалы в каталоге →</Link>
          </div>
        )}
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
                  <p>{material.discipline?.name}</p>
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
    content = (
      <>
        <section aria-labelledby="saved-collections-heading">
          <h2 id="saved-collections-heading">Сохранённые подборки</h2>
          {savedCollections.length === 0 && (
            <div className={styles.empty}>
              <p>Вы пока не сохранили ни одной подборки.</p>
              <Link to="/collections">Посмотреть публичные подборки →</Link>
            </div>
          )}
          <ul className={styles.collections}>
            {savedCollections.map((collection) => (
              <CollectionListItem key={collection.id} collection={collection} />
            ))}
          </ul>
        </section>
        {contributedCollections.length > 0 && (
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
        )}
      </>
    );
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
          aria-current={tab === "saved" ? "page" : undefined}
          onClick={() => changeTab("saved")}
        >
          Сохранённое ({savedMaterials.length})
        </button>
        <button
          type="button"
          aria-current={tab === "collections" ? "page" : undefined}
          onClick={() => changeTab("collections")}
        >
          Подборки ({savedCollectionIds.length})
        </button>
      </nav>
      <div className={styles.content}>{content}</div>
    </div>
  );
}
