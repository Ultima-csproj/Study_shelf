import historyStyles from "./MaterialHistoryPage.module.css";
import { StatusBadge } from "../../components/common/StatusBadge/StatusBadge";
import { useNavigationContext } from "../../hooks/useNavigationContext";
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
import { Pagination } from "../../components/common/Pagination/Pagination";
import common from "../InformationPage/InformationPage.module.css";
import styles from "../../components/materials/shared/Panel.module.css";

export function MaterialHistoryPage() {
  const { back } = useNavigationContext("/catalog");
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
      <header className={historyStyles.header}>
        <h1>История публикаций</h1>
        <p>
          Опубликованные версии материала. Выберите редакцию, чтобы посмотреть
          её содержание.
        </p>
        <nav className={historyStyles.links} aria-label="Вернуться в раздел">
          <Link to={back.path}>← {back.label}</Link>
        </nav>
      </header>
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
      {result?.revisions.items.length === 0 && (
        <p>Опубликованных версий пока нет.</p>
      )}
      {result && (
        <>
          <ul className={historyStyles.list}>
            {result.revisions.items.map((revision) => (
              <li className={historyStyles.item} key={revision.id}>
                <div>
                  <Link
                    to={
                      "/materials/" +
                      id +
                      "/history/" +
                      revision.id +
                      location.search
                    }
                  >
                    {revision.title} · Версия {revision.number}
                  </Link>
                  <time
                    className={historyStyles.date}
                    dateTime={revision.publishedAt ?? undefined}
                  >
                    {revision.publishedAt
                      ? exactPublication(revision.publishedAt)
                      : ""}
                  </time>
                </div>
                {revision.id === result.currentRevisionId && (
                  <StatusBadge state="published" label="Текущая версия" />
                )}
              </li>
            ))}
          </ul>
          <Pagination
            page={page}
            pageCount={result.revisions.pageCount}
            onChange={changePage}
            loading={loading}
            label="Страницы истории"
          />
        </>
      )}
    </main>
  );
}
