import { useNavigationContext } from "../../hooks/useNavigationContext";
import { useCallback } from "react";
import { Link, useSearchParams } from "react-router-dom";
import { getAuthorMaterials } from "../../api/materialWorkspace";
import { getJson } from "../../api/http";
import type { MaterialDetail } from "../../types/material";
import { useApiResource } from "../../hooks/useApiResource";
import { Pagination } from "../../components/common/Pagination/Pagination";
import styles from "./ProfilePublications.module.css";
import layoutStyles from "./ProfilePage.module.css";
import { FileTextIcon } from "@phosphor-icons/react/dist/csr/FileText";
import { materialTypeLabels } from "../../utils/materialLabels";

export default function ProfilePublications() {
  const { linkTo } = useNavigationContext("/profile");
  const [params, setParams] = useSearchParams();
  const requestedPage = Number(params.get("page") ?? 1);
  const page =
    Number.isSafeInteger(requestedPage) && requestedPage > 0
      ? requestedPage
      : 1;
  const load = useCallback(
    async (signal: AbortSignal) => {
      const result = await getAuthorMaterials(
        { page, q: "", state: "published", order: "newest" },
        signal,
      );
      const items = await Promise.all(
        result.items.map((item) =>
          getJson<MaterialDetail>(`/api/me/materials/${item.id}`, signal),
        ),
      );
      return { ...result, items };
    },
    [page],
  );
  const resource = useApiResource(load, "Не удалось загрузить публикации.");

  return (
    <section className={styles.publications}>
      <div className={layoutStyles.heading}>
        <div>
          <h1>Публикации</h1>
          <p>Опубликованные материалы, доступные в вашей рабочей области.</p>
        </div>
        <Link className={layoutStyles.primary} to={linkTo("/author/materials")}>
          Управлять материалами →
        </Link>
      </div>
      {resource.loading && <p role="status">Загружаем публикации…</p>}
      {resource.error && (
        <div role="alert">
          <p>{resource.error}</p>
          <button type="button" onClick={resource.retry}>
            Повторить
          </button>
        </div>
      )}
      {resource.result?.items.length === 0 && (
        <div className={styles.section}>
          <h2>Публикаций пока нет</h2>
          <p>Начните с черновика и отправьте готовый материал на проверку.</p>
          <Link
            className={layoutStyles.primary}
            to={linkTo("/author/materials/new")}
          >
            Создать материал
          </Link>
        </div>
      )}
      <div className={styles.publicationCards}>
        {resource.result?.items.map((item) => (
          <article className={styles.publicationCard} key={item.id}>
            <FileTextIcon
              className={styles.publicationIcon}
              size={32}
              aria-hidden="true"
            />
            <span className={styles.eyebrow}>
              {materialTypeLabels[item.type]} · Опубликован
              {item.sourceYear !== null && ` · ${item.sourceYear}`}
            </span>
            <h2>
              <Link to={linkTo(`/materials/${item.id}`)}>{item.title}</Link>
            </h2>
            <p>{item.description}</p>
            <div className={styles.publicationTags}>
              {item.tags.map((tag) => (
                <span key={tag}>{tag}</span>
              ))}
            </div>
            <Link to={linkTo(`/materials/${item.id}`)}>
              Посмотреть материал →
            </Link>
          </article>
        ))}
      </div>
      {resource.result && (
        <Pagination
          page={page}
          pageCount={resource.result.pageCount}
          onChange={(next) =>
            setParams({ tab: "publications", page: String(next) })
          }
        />
      )}
    </section>
  );
}
