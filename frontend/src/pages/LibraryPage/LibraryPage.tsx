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
import { ProfileLayout } from "../ProfilePage/ProfileLayout";
import { Pagination } from "../../components/common/Pagination/Pagination";
import { Badge } from "../../components/common/Badge/Badge";
import { BookmarkSimpleIcon } from "@phosphor-icons/react/dist/csr/BookmarkSimple";
import { BooksIcon } from "@phosphor-icons/react/dist/csr/Books";
import { FileTextIcon } from "@phosphor-icons/react/dist/csr/FileText";
import { authorLabel } from "../../utils/materialLabels";
import {
  locationPath,
  materialPath,
  withNavigationOrigin,
} from "../../utils/navigation";

export function LibraryPage() {
  const { libraryVersion, viewer } = useViewer();
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

  const selectTab = (tab: string) => setParams({ tab });
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
            <span className={styles.itemIcon}>
              <FileTextIcon size={26} aria-hidden="true" />
            </span>
            <div className={styles.itemBody}>
              {material ? (
                <h3>
                  <Link to={path}>{material.title}</Link>
                </h3>
              ) : (
                <p>Материал недоступен</p>
              )}
              {material && (
                <div className={styles.metadata}>
                  <Badge type={material.type} />
                  <span>{authorLabel(material.credits)}</span>
                  {material.sourceYear !== null && (
                    <span>{material.sourceYear}</span>
                  )}
                </div>
              )}
              {material && (
                <p className={styles.description}>{material.description}</p>
              )}
            </div>
            <SaveAction
              id={slot.id}
              compact
              outlined
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
            <span className={styles.itemIcon}>
              <BooksIcon size={26} aria-hidden="true" />
            </span>
            <div className={styles.itemBody}>
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
              compact
              outlined
              kind="collection"
            />
          </li>
        );
      });
    }
    content = (
      <>
        {result.page.total === 0 && (
          <div className={styles.empty}>
            <BookmarkSimpleIcon size={36} aria-hidden="true" />
            <h2>Пока ничего нет</h2>
            <p>
              Сохраняйте полезные материалы и подборки, чтобы быстро
              возвращаться к ним.
            </p>
            <Link to="/catalog">Перейти в каталог →</Link>
          </div>
        )}
        {result.page.total > 0 && (
          <p className={styles.resultCount}>Всего: {result.page.total}</p>
        )}
        <ul className={styles.materials}>{items}</ul>
        <Pagination
          page={page}
          pageCount={result.page.pageCount}
          onChange={selectPage}
          label="Страницы библиотеки"
        />
      </>
    );
  }
  if (viewer.kind !== "authenticated") {
    return null;
  }
  return (
    <ProfileLayout user={viewer.user}>
      <div className={styles.page}>
        <header className={styles.heading}>
          <div>
            <h1>Библиотека</h1>
            <p>Ваша подборка полезного для учёбы.</p>
          </div>
          <Link
            className={styles.manage}
            to={withNavigationOrigin("/library/collections", parentPath)}
          >
            <BooksIcon size={18} aria-hidden="true" />
            Мои подборки
          </Link>
        </header>
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
    </ProfileLayout>
  );
}
