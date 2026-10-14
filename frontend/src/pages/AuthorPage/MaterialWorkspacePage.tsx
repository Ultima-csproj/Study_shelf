import { useNavigationContext } from "../../hooks/useNavigationContext";
import { RevisionEditorPanel } from "../../components/materials/revisions/RevisionEditorPanel";
import { useCallback, useState } from "react";
import {
  Link,
  useLocation,
  useParams,
  useSearchParams,
} from "react-router-dom";
import { useApiResource } from "../../hooks/useApiResource";
import { useMaterialAction } from "../../hooks/useMaterialAction";
import {
  getWorkspace,
  getRevision,
  forkRevision,
  revisionLabels,
} from "../../api/materialWorkspace";
import { MaterialStateControls } from "../../components/materials/access/MaterialStateControls";
import { CollaboratorsPanel } from "../../components/materials/access/CollaboratorsPanel";
import { MaterialPlacementsPanel } from "../../components/materials/placements/MaterialPlacementsPanel";
import { MaterialEventsPanel } from "../../components/materials/history/MaterialEventsPanel";
import { VersionNavigator } from "../../components/materials/revisions/VersionNavigator";
import {
  PublicationStatus,
  type PublicationTone,
} from "../../components/materials/status/PublicationStatus";
import styles from "./MaterialWorkspacePage.module.css";
import panelStyles from "../../components/materials/shared/Panel.module.css";

export function MaterialWorkspacePage() {
  const { back, linkTo } = useNavigationContext("/author/materials");
  const location = useLocation();
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
      const result = await forkRevision(
        material,
        source,
        "Подготовка новой версии материала",
      );
      choose(result.revisionId);
    });
  };
  const ready = !action.busy;
  const material = workspace.result?.material;
  let publicationLabel = "Не опубликован";
  let publicationTone: PublicationTone = "archived";
  if (material?.publication === "published") {
    publicationLabel = "Опубликован";
    publicationTone = "published";
  }
  if (material?.lifecycle === "archived") {
    publicationLabel = "В архиве";
    publicationTone = "archived";
  }
  let workingLabel = "";
  let workingTone: PublicationTone = "draft";
  if (material?.workingRevisionState === "draft") {
    workingLabel =
      material.publication === "published"
        ? "Есть черновик новой редакции"
        : "Черновик";
  } else if (material?.workingRevisionState === "pending_moderation") {
    workingLabel =
      material.publication === "published"
        ? "Новая редакция на проверке"
        : "На проверке";
    workingTone = "pending";
  }
  const revision = version.result?.revision;
  let revisionHeading = "Редакция материала";
  let revisionHelp = "Выберите редакцию в разделе «Версии и история».";
  if (revision?.state === "draft") {
    revisionHeading =
      revision.baseRevisionId === null
        ? "Редактирование черновика"
        : "Подготовка новой редакции";
    revisionHelp =
      revision.baseRevisionId === null
        ? "Сохраните черновик или отправьте его на модерацию."
        : "Публичная версия продолжит действовать, пока новую редакцию не одобрят.";
  } else if (revision?.state === "pending_moderation") {
    revisionHeading = "Редакция на проверке";
    revisionHelp = "Материал ожидает решения модератора.";
  } else if (revision) {
    revisionHeading = "Просмотр редакции";
    revisionHelp = "Эта редакция доступна только для просмотра.";
  }
  const [eventsLoaded, setEventsLoaded] = useState(false);
  return (
    <div
      data-material-workspace
      className={`${panelStyles.page} ${styles.workspacePage}`}
    >
      <div className={styles.workspaceTop}>
        <div className={styles.workspaceHeading}>
          <Link className={styles.backLink} to={back.path}>
            ← {back.label}
          </Link>
          <h1>{material?.title ?? "Рабочая область материала"}</h1>
          {material && (
            <>
              <div className={styles.workspaceStatuses}>
                <PublicationStatus tone={publicationTone}>
                  {publicationLabel}
                </PublicationStatus>
                {workingLabel && (
                  <PublicationStatus tone={workingTone}>
                    {workingLabel}
                  </PublicationStatus>
                )}
              </div>
              <p className={styles.workspaceMeta}>
                Владелец: {material.ownerName}
              </p>
            </>
          )}
        </div>
        {material?.publishedRevisionId !== null && material && (
          <div className={styles.workspaceLinks}>
            <Link to={linkTo(`/materials/${id}`)}>Открыть публикацию</Link>
            <Link to={linkTo(`/materials/${id}/history`)}>
              История публикаций
            </Link>
          </div>
        )}
        {material &&
          workspace.result?.capabilities.edit &&
          material.workingRevisionId === null && (
            <button
              type="button"
              className={styles.primaryAction}
              disabled={!ready}
              onClick={fork}
            >
              Подготовить новую версию
            </button>
          )}
      </div>
      {location.state?.notice && !action.notice && (
        <p className={panelStyles.banner} role="status">
          {location.state.notice}
        </p>
      )}
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
        <p className={styles.workspaceNotice} role="status">
          {action.notice}{" "}
          <button type="button" onClick={changed}>
            Обновить данные
          </button>
        </p>
      )}
      {workspace.result && (
        <>
          <div className={styles.workspaceGrid}>
            <section
              className={styles.editorColumn}
              aria-label="Редактирование редакции"
            >
              <section className={styles.revisionSection}>
                <header className={styles.sectionHeading}>
                  <div>
                    <p className={styles.eyebrow}>Рабочая редакция</p>
                    <h2>{revisionHeading}</h2>
                    <p>{revisionHelp}</p>
                  </div>
                  {revision && (
                    <span className={styles.versionLabel}>
                      Версия {revision.number} ·{" "}
                      {revisionLabels[revision.state] ?? revision.state}
                    </span>
                  )}
                </header>
                {version.loading && (
                  <p className={styles.workspaceNotice} role="status">
                    Загружаем редакцию…
                  </p>
                )}
                {version.error && (
                  <p className={styles.workspaceNotice} role="alert">
                    {version.error}{" "}
                    <button type="button" onClick={version.retry}>
                      Повторить
                    </button>
                  </p>
                )}
                {version.result && (
                  <RevisionEditorPanel
                    key={version.result.revision.id}
                    value={version.result}
                    workspace={workspace.result}
                    reason={reason}
                    action={action}
                    changed={changed}
                    onChoose={choose}
                  />
                )}
                {!version.loading && !version.error && !version.result && (
                  <p className={styles.workspaceNotice}>
                    У материала пока нет редакции для просмотра.
                  </p>
                )}
              </section>
            </section>
            <aside
              className={styles.contextColumn}
              aria-label="Управление материалом"
            >
              <details
                className={`${panelStyles.card} ${styles.workspaceDisclosure}`}
              >
                <summary>
                  <span>Версии и история</span>
                  <span className={styles.disclosureMeta}>
                    Редакций: {workspace.result.revisions.total}
                  </span>
                </summary>
                <div className={styles.disclosureBody}>
                  <VersionNavigator
                    revisions={workspace.result.revisions}
                    selected={selected}
                    page={page}
                    loading={workspace.loading}
                    onChoose={choose}
                    onPage={changePage}
                  />
                </div>
              </details>
              <details
                className={`${panelStyles.card} ${styles.workspaceDisclosure}`}
              >
                <summary>
                  <span>Доступность и ограничения</span>
                  <span className={styles.disclosureMeta}>
                    {material?.visibility === "public"
                      ? "Публичный доступ"
                      : "Приватный доступ"}
                  </span>
                </summary>
                <div className={styles.disclosureBody}>
                  <label className={panelStyles.label}>
                    Причина изменения доступности или размещения
                    <textarea
                      minLength={3}
                      maxLength={2000}
                      value={reason}
                      onChange={(event) => setReason(event.target.value)}
                    />
                  </label>
                  <MaterialStateControls
                    key={workspace.result.material.version}
                    workspace={workspace.result}
                    busy={action.busy}
                    reason={reason}
                    run={action.run}
                  />
                  <MaterialPlacementsPanel
                    material={workspace.result.material}
                    placements={workspace.result.placements}
                    busy={action.busy}
                    reason={reason}
                    run={action.run}
                  />
                </div>
              </details>
              {workspace.result.capabilities.collaborators && (
                <details
                  className={`${panelStyles.card} ${styles.workspaceDisclosure}`}
                >
                  <summary>
                    <span>Приглашённые пользователи и права</span>
                  </summary>
                  <div className={styles.disclosureBody}>
                    <CollaboratorsPanel
                      key={"collaborators:" + workspace.result.material.version}
                      material={workspace.result.material}
                      busy={action.busy}
                      run={action.run}
                    />
                  </div>
                </details>
              )}
            </aside>
          </div>
          <details
            className={`${panelStyles.card} ${styles.workspaceDisclosure} ${styles.activityDisclosure}`}
            onToggle={(event) => {
              if (event.currentTarget.open) {
                setEventsLoaded(true);
              }
            }}
          >
            <summary>
              <span>История действий</span>
              <span className={styles.disclosureMeta}>
                Журнал изменений материала
              </span>
            </summary>
            {eventsLoaded && (
              <div className={styles.disclosureBody}>
                <MaterialEventsPanel
                  key={"events:" + revisionCounter}
                  id={id}
                />
              </div>
            )}
          </details>
        </>
      )}
    </div>
  );
}
