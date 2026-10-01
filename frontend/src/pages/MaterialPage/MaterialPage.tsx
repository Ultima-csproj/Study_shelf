import { Fragment } from "react";
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
  const originCollectionId = origin?.kind === "collection" ? origin.id : null;
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
  const { discipline, teacher } = material;
  const author = material.authorName ?? teacher?.name ?? "Автор не указан";
  const disciplinePath = discipline ? `/disciplines/${discipline.slug}` : null;
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
            {author} · {material.course} курс · {material.year}
          </p>
          {disciplinePath && (
            <Link className={styles.disciplineLink} to={disciplinePath}>
              Перейти к дисциплине: {discipline?.name}
            </Link>
          )}
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
        <p className={styles.availability}>
          Файл материала пока не подключён. Здесь доступна публичная информация
          о нём.
        </p>
      </div>
    </main>
  );
}
