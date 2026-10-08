import { useCallback } from "react";
import { Link, useLocation, useParams } from "react-router-dom";
import { getPublishedRevision } from "../../api/materialWorkspace";
import { RevisionContent } from "../../components/materialWorkspace/RevisionContent";
import { useViewer } from "../../hooks/useViewer";
import { useApiResource } from "../../hooks/useApiResource";
import { materialContext } from "../../utils/materialContext";
import common from "../InformationPage/InformationPage.module.css";
import styles from "../../components/materialWorkspace/Workspace.module.css";

export function PublishedRevisionPage() {
  const params = useParams();
  const location = useLocation();
  const material = Number(params.materialId);
  const revision = Number(params.revisionId);
  const { viewer } = useViewer();
  const account =
    viewer.kind === "authenticated" ? viewer.user.id : viewer.kind;
  const authenticated = viewer.kind === "authenticated";
  const context = materialContext(location.search);
  const load = useCallback(
    (signal: AbortSignal) => {
      void account;
      return getPublishedRevision(
        material,
        revision,
        authenticated,
        context,
        signal,
      );
    },
    [material, revision, authenticated, context, account],
  );
  const { result, loading, error, retry } = useApiResource(
    load,
    "Редакция недоступна",
  );
  return (
    <main id="main-content" className={common.page}>
      <Link
        className={common.back}
        to={"/materials/" + material + "/history" + location.search}
      >
        ← История публикаций
      </Link>
      <h1>Опубликованная редакция #{revision}</h1>
      {loading && <p role="status">Загрузка…</p>}
      {error && (
        <p role="alert">
          {error}{" "}
          <button type="button" onClick={retry}>
            Повторить
          </button>
        </p>
      )}
      {result && (
        <>
          {result.material.publishedRevisionId !== revision && (
            <p className={styles.banner}>
              Вы смотрите прежнюю версию.{" "}
              <Link to={"/materials/" + material + location.search}>
                Перейти к текущей публикации
              </Link>
              .
            </p>
          )}
          {result.material.lifecycle === "archived" && (
            <p className={styles.banner}>Материал в архиве.</p>
          )}
          {result.material.publication === "unpublished" && (
            <p className={styles.banner}>
              Материал снят с публикации. Версия доступна вам по действующим
              правам.
            </p>
          )}
          <RevisionContent value={result} />
        </>
      )}
    </main>
  );
}
