import { useCallback, type ReactNode } from "react";
import { Link, useLocation, useSearchParams } from "react-router-dom";
import {
  getSavedMaterials,
  getSavedCollections,
  getContributedCollections,
} from "../../api/library";
import { useApiResource } from "../../hooks/useApiResource";
import { useViewer } from "../../hooks/useViewer";
import { SaveAction } from "../../components/common/SaveAction/SaveAction";
import styles from "./LibraryPage.module.css";
import {
  locationPath,
  materialPath,
  withNavigationOrigin,
} from "../../utils/navigation";

export function LibraryPage() {
  const { libraryVersion } = useViewer();
  const [params, setParams] = useSearchParams();
  const location = useLocation();
  const parentPath = locationPath(location);
  const contributions = params.get("tab") === "contributions";
  const collections = params.get("tab") === "collections" || contributions;
  let tab = "saved";
  if (contributions) {
    tab = "contributions";
  } else if (collections) {
    tab = "collections";
  }
  const requested = Number(params.get("page") ?? "1");
  const page = Number.isSafeInteger(requested) && requested > 0 ? requested : 1;
  const load = useCallback(
    async (signal: AbortSignal) => {
      const version = libraryVersion;
      void version;
      if (contributions) {
        return {
          kind: "collections" as const,
          page: await getContributedCollections(page, signal),
        };
      }
      if (collections) {
        return {
          kind: "collections" as const,
          page: await getSavedCollections(page, signal),
        };
      }
      return {
        kind: "materials" as const,
        page: await getSavedMaterials(page, signal),
      };
    },
    [collections, contributions, page, libraryVersion],
  );
  const { result, error, loading, retry } = useApiResource(
    load,
    "Не удалось загрузить библиотеку",
  );

  const selectTab = (tab: string) => setParams({ tab }, { replace: true });
  const selectPage = (next: number) => setParams({ tab, page: String(next) });
  let content: ReactNode = null;
  if (loading) {
    content = <p role="status">Загружаем библиотеку…</p>;
  } else if (error) {
    content = (
      <div>
        <p role="alert">{error}</p>
        <button type="button" onClick={retry}>
          Повторить
        </button>
      </div>
    );
  } else if (result) {
    let items: ReactNode;
    if (result.kind === "materials") {
      items = result.page.items.map((slot) => {
        const material = slot.material;
        const path = withNavigationOrigin(
          materialPath(slot.id, "api"),
          parentPath,
        );
        return (
          <li key={slot.id}>
            <div>
              {material ? (
                <h3>
                  <Link to={path}>{material.title}</Link>
                </h3>
              ) : (
                <p>Материал недоступен</p>
              )}
            </div>
            <SaveAction
              id={slot.id}
              title={material?.title ?? "Недоступный материал"}
            />
          </li>
        );
      });
    } else {
      items = result.page.items.map((slot) => {
        const target = slot.owned
          ? `/library/collections/${slot.id}`
          : `/collections/${slot.id}`;
        const path = withNavigationOrigin(target, parentPath);
        return (
          <li key={slot.id}>
            <div>
              {slot.title === null ? (
                <p>Подборка недоступна</p>
              ) : (
                <h3>
                  <Link to={path}>{slot.title}</Link>
                </h3>
              )}
              <p>{slot.shortDescription}</p>
            </div>
            <SaveAction
              id={slot.id}
              title={slot.title ?? "Недоступная подборка"}
              kind="collection"
            />
          </li>
        );
      });
    }
    content = (
      <>
        {result.page.total === 0 && <p>Здесь появятся ваши сохранения.</p>}
        <ul className={styles.materials}>{items}</ul>
        <nav aria-label="Страницы библиотеки">
          <button
            type="button"
            disabled={page <= 1}
            onClick={() => selectPage(page - 1)}
          >
            Назад
          </button>
          <span>
            {" "}
            Страница {page} из {Math.max(1, result.page.pageCount)} · Всего{" "}
            {result.page.total}{" "}
          </span>
          <button
            type="button"
            disabled={page >= result.page.pageCount}
            onClick={() => selectPage(page + 1)}
          >
            Далее
          </button>
        </nav>
      </>
    );
  }
  return (
    <div className={styles.page}>
      <h1>Моя библиотека</h1>
      <p className={styles.subtitle}>
        Сохранённые материалы и учебные маршруты.
      </p>
      <Link to="/library/collections">
        Создать и редактировать свои подборки →
      </Link>
      <nav className={styles.tabs} aria-label="Разделы библиотеки">
        <button
          type="button"
          aria-current={!collections ? "page" : undefined}
          onClick={() => selectTab("saved")}
        >
          Материалы
        </button>
        <button
          type="button"
          aria-current={collections && !contributions ? "page" : undefined}
          onClick={() => selectTab("collections")}
        >
          Подборки
        </button>
        <button
          type="button"
          aria-current={contributions ? "page" : undefined}
          onClick={() => selectTab("contributions")}
        >
          С моим участием
        </button>
      </nav>
      <div className={styles.content}>{content}</div>
    </div>
  );
}
