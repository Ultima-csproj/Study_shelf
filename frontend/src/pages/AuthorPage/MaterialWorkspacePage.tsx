import { RevisionEditorPanel } from "../../components/materialWorkspace/RevisionEditorPanel";
import { useCallback, useState } from "react";
import { Link, useParams, useSearchParams } from "react-router-dom";
import { useApiResource } from "../../hooks/useApiResource";
import { useMaterialAction } from "../../hooks/useMaterialAction";
import {
  getWorkspace,
  getRevision,
  forkRevision,
  revisionLabels,
} from "../../api/materialWorkspace";
import { MaterialStateControls } from "../../components/materialWorkspace/MaterialStateControls";
import { CollaboratorsPanel } from "../../components/materialWorkspace/CollaboratorsPanel";
import { MaterialPlacementsPanel } from "../../components/materialWorkspace/MaterialPlacementsPanel";
import { MaterialEventsPanel } from "../../components/materialWorkspace/MaterialEventsPanel";
import styles from "../../components/materialWorkspace/Workspace.module.css";

export function MaterialWorkspacePage() {
  const id = Number(useParams().id);
  const [params, setParams] = useSearchParams();
  const [revisionCounter, setRevisionCounter] = useState(0);
  const [reason, setReason] = useState("");
  const requestedPage = Number(params.get("page") ?? "1");
  const page =
    Number.isSafeInteger(requestedPage) && requestedPage > 0
      ? requestedPage
      : 1;
  const load = useCallback(
    (signal: AbortSignal) => {
      void revisionCounter;
      return getWorkspace(id, page, signal);
    },
    [id, page, revisionCounter],
  );
  const workspace = useApiResource(load, "Не удалось открыть рабочую область");
  const fallback =
    workspace.result?.material.workingRevisionId ??
    workspace.result?.revisions.items[0]?.id ??
    0;
  const selected = Number(params.get("revision") ?? fallback);
  const loadRevision = useCallback(
    (signal: AbortSignal) => {
      void revisionCounter;
      if (selected <= 0) {
        return Promise.resolve(null);
      }
      return getRevision(id, selected, signal);
    },
    [id, selected, revisionCounter],
  );
  const version = useApiResource(loadRevision, "Не удалось загрузить редакцию");
  const changed = useCallback(
    () => setRevisionCounter((value) => value + 1),
    [],
  );
  const action = useMaterialAction(changed);
  const choose = (revision: number) => {
    const next = new URLSearchParams(params);
    next.set("revision", String(revision));
    setParams(next);
  };
  const changePage = (nextPage: number) => {
    const next = new URLSearchParams(params);
    next.set("page", String(nextPage));
    setParams(next);
  };
  const fork = () => {
    if (!workspace.result) {
      return;
    }
    const material = workspace.result.material;
    const source = version.result?.revision.id ?? null;
    void action.run(async () => {
      const result = await forkRevision(material, source, reason);
      choose(result.revisionId);
    });
  };
  const ready = !action.busy && reason.trim().length >= 3;
  return (
    <div className={styles.page}>
      <Link to="/author/materials">← Мои публикации</Link>
      <h1>{workspace.result?.material.title ?? "Рабочая область материала"}</h1>
      {workspace.loading && <p role="status">Загрузка…</p>}
      {workspace.error && (
        <p role="alert">
          {workspace.error}{" "}
          <button type="button" onClick={workspace.retry}>
            Повторить
          </button>
        </p>
      )}
      {action.notice && (
        <p role="status">
          {action.notice}{" "}
          <button type="button" onClick={changed}>
            Обновить данные
          </button>
        </p>
      )}
      {workspace.result && (
        <>
          <p>
            Владелец: {workspace.result.material.ownerName} · Материал #{id}
          </p>
          {workspace.result.material.publishedRevisionId !== null && (
            <p>
              <Link to={"/materials/" + id}>
                Последняя опубликованная редакция
              </Link>{" "}
              ·{" "}
              <Link to={"/materials/" + id + "/history"}>
                История публикаций
              </Link>
            </p>
          )}
          <label className={styles.label}>
            Причина действия
            <textarea
              minLength={3}
              maxLength={2000}
              value={reason}
              onChange={(event) => setReason(event.target.value)}
            />
          </label>
          <div className={styles.actions}>
            {workspace.result.capabilities.edit &&
              workspace.result.material.workingRevisionId === null && (
                <button type="button" disabled={!ready} onClick={fork}>
                  Создать черновик из выбранной редакции
                </button>
              )}
          </div>
          <section className={styles.card}>
            <h2>Редакции</h2>
            <ul className={styles.list}>
              {workspace.result.revisions.items.map((revision) => (
                <li key={revision.id}>
                  <button type="button" onClick={() => choose(revision.id)}>
                    #{revision.id}: {revision.title}
                  </button>{" "}
                  — {revisionLabels[revision.state] ?? revision.state}
                </li>
              ))}
            </ul>
            <nav className={styles.actions} aria-label="Страницы редакций">
              <button
                type="button"
                disabled={page <= 1}
                onClick={() => changePage(page - 1)}
              >
                Назад
              </button>
              <span>
                {page} / {Math.max(1, workspace.result.revisions.pageCount)}
              </span>
              <button
                type="button"
                disabled={page >= workspace.result.revisions.pageCount}
                onClick={() => changePage(page + 1)}
              >
                Далее
              </button>
            </nav>
          </section>
          {version.loading && <p role="status">Загружаем редакцию…</p>}
          {version.error && (
            <p role="alert">
              {version.error}{" "}
              <button type="button" onClick={version.retry}>
                Повторить
              </button>
            </p>
          )}
          {version.result && (
            <RevisionEditorPanel
              value={version.result}
              workspace={workspace.result}
              reason={reason}
              action={action}
              changed={changed}
              onChoose={choose}
            />
          )}
          <MaterialStateControls
            key={workspace.result.material.version}
            workspace={workspace.result}
            busy={action.busy}
            reason={reason}
            run={action.run}
          />
          {workspace.result.capabilities.collaborators && (
            <CollaboratorsPanel
              key={"collaborators:" + workspace.result.material.version}
              material={workspace.result.material}
              busy={action.busy}
              run={action.run}
            />
          )}
          <MaterialPlacementsPanel
            material={workspace.result.material}
            placements={workspace.result.placements}
            busy={action.busy}
            reason={reason}
            run={action.run}
          />
          <MaterialEventsPanel key={"events:" + revisionCounter} id={id} />
        </>
      )}
    </div>
  );
}
