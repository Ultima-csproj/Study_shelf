import { useCallback } from "react";
import {
  Link,
  useLocation,
  useParams,
  useSearchParams,
} from "react-router-dom";
import { getPublishedHistory } from "../../api/materialWorkspace";
import { useApiResource } from "../../hooks/useApiResource";
import { useViewer } from "../../hooks/useViewer";
import { materialContext } from "../../utils/materialContext";
import { exactPublication } from "../../utils/materialLabels";
import common from "../InformationPage/InformationPage.module.css";
import styles from "../../components/materialWorkspace/Workspace.module.css";

export function MaterialHistoryPage() {
  const id = Number(useParams().materialId);
  const location = useLocation();
  const [params, setParams] = useSearchParams();
  const { viewer } = useViewer();
  const account =
    viewer.kind === "authenticated" ? viewer.user.id : viewer.kind;
  const authenticated = viewer.kind === "authenticated";
  const page = Math.max(1, Number(params.get("page") ?? "1") || 1);
  const context = materialContext(location.search);
  const load = useCallback(
    (signal: AbortSignal) => {
      void account;
      return getPublishedHistory(id, page, authenticated, context, signal);
    },
    [id, page, authenticated, context, account],
  );
  const { result, loading, error, retry } = useApiResource(
    load,
    "История недоступна",
  );
  const changePage = (nextPage: number) => {
    const next = new URLSearchParams(params);
    next.set("page", String(nextPage));
    setParams(next);
  };
  return (
    <main id="main-content" className={common.page}>
      <Link className={common.back} to={"/materials/" + id + location.search}>
        ← Текущая публикация
      </Link>
      <h1>История опубликованных редакций</h1>
      {loading && <p role="status">Загрузка…</p>}
      {error && (
        <p role="alert">
          {error}{" "}
          <button type="button" onClick={retry}>
            Повторить
          </button>
        </p>
      )}
      {result?.lifecycle === "archived" && (
        <p className={styles.banner}>
          Материал в архиве. Доступные опубликованные версии сохранены.
        </p>
      )}
      {result && (
        <>
          <ul className={styles.list}>
            {result.revisions.items.map((revision) => (
              <li key={revision.id}>
                <Link
                  to={
                    "/materials/" +
                    id +
                    "/history/" +
                    revision.id +
                    location.search
                  }
                >
                  {revision.title} · #{revision.id}
                </Link>{" "}
                —{" "}
                {revision.publishedAt
                  ? exactPublication(revision.publishedAt)
                  : ""}
                {revision.id === result.currentRevisionId &&
                  " · текущая редакция"}
              </li>
            ))}
          </ul>
          <nav className={styles.actions} aria-label="Страницы истории">
            <button
              type="button"
              disabled={page <= 1}
              onClick={() => changePage(page - 1)}
            >
              Назад
            </button>
            <span>
              {page} / {Math.max(1, result.revisions.pageCount)}
            </span>
            <button
              type="button"
              disabled={page >= result.revisions.pageCount}
              onClick={() => changePage(page + 1)}
            >
              Далее
            </button>
          </nav>
        </>
      )}
    </main>
  );
}
