import { useEffect, type ReactNode } from "react";
import { Link, useLocation, useParams } from "react-router-dom";
import { ArrowRightIcon } from "@phosphor-icons/react/dist/csr/ArrowRight";
import { LinkIcon } from "@phosphor-icons/react/dist/csr/Link";
import { CollectionSectionEntries } from "../../components/collections/CollectionSectionEntries/CollectionSectionEntries";
import { SaveAction } from "../../components/common/SaveAction/SaveAction";
import { useCollection } from "../../hooks/useCollections";
import { useViewer } from "../../hooks/useViewer";
import {
  collectionMetadata,
  compilerNames,
  difficultyLabels,
} from "../../utils/collectionLabels";
import styles from "./CollectionPage.module.css";
import {
  collectionsForDiscipline,
  navigationOrigin,
} from "../../utils/navigation";

export function CollectionPage() {
  const { collectionId = "" } = useParams();
  const location = useLocation();
  const {
    result: collection,
    loading,
    error,
    retry,
  } = useCollection(collectionId);
  const { viewer } = useViewer();
  const origin = navigationOrigin(location.search);
  let parentPath = "/collections";
  let parentLabel = "Подборки";
  let backLabel = "К списку подборок";

  if (origin?.kind === "collections" || origin?.kind === "library") {
    parentPath = origin.path;
  }

  if (origin?.kind === "library") {
    parentLabel = "Моя библиотека";
    backLabel = "Моя библиотека";
  }

  const requestedSection = location.hash.slice(1);

  useEffect(() => {
    if (!collection || loading || !requestedSection) {
      return;
    }

    const validSection = collection.sections.some((section) => {
      return `route-${section.id}` === requestedSection;
    });

    if (validSection) {
      document
        .getElementById(requestedSection)
        ?.scrollIntoView({ block: "start" });
    }
  }, [collection, loading, requestedSection]);

  if (loading) {
    return (
      <main id="main-content" className={styles.state}>
        <p role="status">Загружаем подборку…</p>
      </main>
    );
  }

  if (error) {
    return (
      <main id="main-content" className={styles.state}>
        <h1>Не удалось загрузить подборку</h1>
        <p role="alert">{error}</p>
        <button type="button" onClick={retry}>
          Повторить загрузку
        </button>
      </main>
    );
  }

  if (!collection) {
    return (
      <main id="main-content" className={styles.state}>
        <h1>Подборка не найдена</h1>
        <p>Она могла быть удалена или закрыта для публичного просмотра.</p>
        <Link to={parentPath}>{backLabel} →</Link>
      </main>
    );
  }

  const metadata = collectionMetadata(collection);
  let difficulty: string | null = null;

  if (collection.difficulty) {
    difficulty = difficultyLabels[collection.difficulty];
  }

  let fullMetadata = metadata;

  if (difficulty) {
    fullMetadata = `${metadata} · ${difficulty}`;
  }

  const names = compilerNames(collection.contributors);
  const validSection = collection.sections.some((section) => {
    return `route-${section.id}` === requestedSection;
  });
  let currentSection = `route-${collection.sections[0]?.id}`;

  if (validSection) {
    currentSection = requestedSection;
  }

  let disciplinePath: string | null = null;

  if (collection.discipline) {
    disciplinePath = collectionsForDiscipline(collection.discipline.id);
  }

  const sectionNavigationLinks: ReactNode[] = [];

  for (const section of collection.sections) {
    const id = `route-${section.id}`;
    const selected = currentSection === id;
    let className = "";
    let current: "location" | undefined = undefined;

    if (selected) {
      className = styles.selected;
      current = "location";
    }

    const sectionLink = (
      <a
        key={section.id}
        href={`#${id}`}
        className={className}
        aria-current={current}
      >
        <span>{section.position}.</span>
        {section.title}
      </a>
    );

    sectionNavigationLinks.push(sectionLink);
  }

  const navigation = (
    <nav aria-label="Разделы подборки">{sectionNavigationLinks}</nav>
  );

  let disciplineBreadcrumb: ReactNode = null;

  if (disciplinePath && collection.discipline) {
    disciplineBreadcrumb = (
      <>
        <span aria-hidden="true">/</span>
        <Link to={disciplinePath} title="Подборки по этой дисциплине">
          {collection.discipline.name}
        </Link>
      </>
    );
  }

  let guestSaveHint: ReactNode = null;

  if (viewer.kind === "guest") {
    guestSaveHint = <p>Сохранение — после входа.</p>;
  }

  let emptySectionsMessage: ReactNode = null;

  if (collection.sections.length === 0) {
    emptySectionsMessage = (
      <p className={styles.empty}>В этой подборке пока нет материалов.</p>
    );
  }

  return (
    <div className={styles.layout}>
      <aside className={styles.sidebar}>
        <div className={styles.sticky}>
          <div className={styles.desktopNavigation}>
            <h2>В этой подборке</h2>
            {navigation}
          </div>
          <details className={styles.mobileNavigation}>
            <summary>В этой подборке</summary>
            {navigation}
          </details>
          <Link to={parentPath} className={styles.allCollections}>
            <LinkIcon size={21} aria-hidden="true" />
            {backLabel}
            <ArrowRightIcon size={19} aria-hidden="true" />
          </Link>
        </div>
      </aside>
      <main id="main-content" className={styles.content} tabIndex={-1}>
        <nav aria-label="Хлебные крошки" className={styles.breadcrumbs}>
          <Link to={parentPath}>{parentLabel}</Link>
          {disciplineBreadcrumb}
        </nav>
        <div className={styles.heading}>
          <div className={styles.intro}>
            <h1>{collection.title}</h1>
            <p className={styles.description}>{collection.description}</p>
            <p className={styles.metadata}>{fullMetadata}</p>
            <p className={styles.contributors}>Составили: {names}</p>
          </div>
          <div className={styles.save}>
            <SaveAction
              id={collection.id}
              kind="collection"
              title={collection.title}
              outlined
            />
            {guestSaveHint}
          </div>
        </div>
        <div className={styles.purpose}>
          <section aria-labelledby="collection-purpose">
            <h2 id="collection-purpose">Зачем эта подборка</h2>
            <p>{collection.purpose}</p>
          </section>
          <section aria-labelledby="collection-audience">
            <h2 id="collection-audience">Для кого</h2>
            <p>{collection.targetAudience}</p>
          </section>
        </div>
        {emptySectionsMessage}
        {collection.sections.map((section) => {
          const id = `route-${section.id}`;
          const headingId = `${id}-heading`;

          return (
            <section
              key={section.id}
              id={id}
              aria-labelledby={headingId}
              className={styles.section}
            >
              <h2 id={headingId}>
                <span>{section.position}</span>
                {section.title}
              </h2>
              <CollectionSectionEntries
                collectionId={collection.id}
                section={section}
                sectionId={id}
              />
            </section>
          );
        })}
      </main>
    </div>
  );
}
