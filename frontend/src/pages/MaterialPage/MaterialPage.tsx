import { Fragment } from "react";
import { authorLabel, contributorRoles, exactPublication, publicationLabel } from "../../utils/materialLabels";
import { Link, useLocation, useParams } from "react-router-dom";
import { useMaterial } from "../../hooks/useMaterial";
import { MaterialPreview } from "../../components/common/MaterialPreview/MaterialPreview";
import { Badge } from "../../components/common/Badge/Badge";
import { SaveAction } from "../../components/common/SaveAction/SaveAction";
import common from "../InformationPage/InformationPage.module.css";
import styles from "./MaterialPage.module.css";
import { useCollection } from "../../hooks/useCollections";
import { materialSource, navigationOrigin } from "../../utils/navigation";
import { materialNavigation } from "../../utils/materialNavigation";

export function MaterialPage() {
  const { materialId = "" } = useParams();
  const location = useLocation();
  const origin = navigationOrigin(location.search);
  let originCollectionId: string | null = null;

  if (origin?.kind === "collection") {
    originCollectionId = origin.id;
  }

  const { result: originCollection, loading: originLoading } =
    useCollection(originCollectionId);
  const source = materialSource(location.search);
  const { material, loading, error, retry } = useMaterial(materialId, source);
  const navigation = materialNavigation(origin, material, originCollection);
  const contextLoading = originCollectionId !== null && originLoading;

  if (loading || contextLoading) {
    return (
      <main id="main-content" className={common.page} aria-busy="true">
        <p role="status">Загружаем материал…</p>
      </main>
    );
  }

  if (error) {
    return (
      <main id="main-content" className={common.page}>
        <p role="alert">{error}</p>
        <Link className={common.back} to={navigation.backPath}>
          ← {navigation.backLabel}
        </Link>
        <button className={common.primary} onClick={retry}>
          Повторить загрузку
        </button>
      </main>
    );
  }

  if (!material) {
    return (
      <main id="main-content" className={common.page}>
        <h1>Материал не найден</h1>
        <Link className={common.back} to={navigation.backPath}>
          {navigation.backLabel}
        </Link>
      </main>
    );
  }

  const author = authorLabel(material.credits);
  const metadataParts = [author];
  if (material.sourceYear !== null) {
    metadataParts.push("Год источника: " + material.sourceYear);
  }
  const metadata = metadataParts.join(" · ");
  const dateLabel = publicationLabel(material.firstPublishedAt, material.publishedAt);
  const publishedDate = exactPublication(material.publishedAt);
  const placements = material.placements.map((placement) => {
    const discipline = placement.discipline;
    let courseLabel = "";
    if (placement.course !== null) {
      courseLabel = " · " + placement.course + " курс";
    }
    let responsibleLabel = "";
    if (placement.responsible) {
      responsibleLabel = " · Ответственный: " + placement.responsible.name;
    }
    return (
      <p key={placement.id}>
        <Link className={styles.disciplineLink} to={"/disciplines/" + discipline.id}>
          {discipline.name}
        </Link>
        {courseLabel}{responsibleLabel}
      </p>
    );
  });

  return (
    <main id="main-content" className={common.page}>
      <nav className={styles.breadcrumbs} aria-label="Хлебные крошки">
        {navigation.crumbs.map((crumb) => (
          <Fragment key={crumb.path}>
            <Link to={crumb.path}>{crumb.label}</Link>
            <span aria-hidden="true">/</span>
          </Fragment>
        ))}
        <span aria-current="page">{material.title}</span>
      </nav>
      <Link className={common.back} to={navigation.backPath}>
        ← {navigation.backLabel}
      </Link>
      <div className={styles.summary}>
        <MaterialPreview material={material} />
        <div>
          <Badge type={material.type} />
          <h1>{material.title}</h1>
          <p>
            {metadata}
          </p>
          <time dateTime={material.publishedAt}>{dateLabel}: {publishedDate}</time>
          {placements}
          <SaveAction
            id={material.id}
            title={material.title}
            source={material.source}
          />
        </div>
      </div>
      <div className={common.notice}>
        <h2>О материале</h2>
        <p>{material.description}</p>
        <h2>Участники материала</h2>
        <ul>
          {material.credits.map((credit) => (
            <li key={String(credit.contributorId) + ":" + credit.role}>
              {contributorRoles[credit.role]}: {credit.displayName}
            </li>
          ))}
        </ul>
        <p className={styles.availability}>
          Файл материала пока не подключён. Здесь доступна публичная информация
          о нём.
        </p>
      </div>
    </main>
  );
}
