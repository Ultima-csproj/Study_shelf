import { useNavigationContext } from "../../../hooks/useNavigationContext";
import { Link } from "react-router-dom";
import { FileTextIcon } from "@phosphor-icons/react/dist/csr/FileText";
import { ArrowRightIcon } from "@phosphor-icons/react/dist/csr/ArrowRight";
import { DotsThreeIcon } from "@phosphor-icons/react/dist/csr/DotsThree";
import { BookOpenIcon } from "@phosphor-icons/react/dist/csr/BookOpen";
import { PlayIcon } from "@phosphor-icons/react/dist/csr/Play";
import { CodeIcon } from "@phosphor-icons/react/dist/csr/Code";
import type { MaterialHeader } from "../../../api/materialWorkspace";
import {
  materialTypeLabels,
  exactPublication,
  relativePublication,
} from "../../../utils/materialLabels";
import { PublicationStatus } from "../status/PublicationStatus";
import styles from "./AuthorMaterialCard.module.css";

export function AuthorMaterialCard({
  material,
  userId,
}: {
  material: MaterialHeader;
  userId: number;
}) {
  const { linkTo } = useNavigationContext("/profile");
  const pending = material.workingRevisionState === "pending_moderation";
  const draft = material.workingRevisionState === "draft";
  const published = material.publication === "published";
  const archived = material.lifecycle === "archived";
  const workspace = linkTo(`/author/materials/${material.id}`);
  let action = "Открыть материал";
  if (draft) {
    action = published
      ? "Продолжить новую версию"
      : "Продолжить редактирование";
  } else if (pending) {
    action = "Посмотреть отправленную версию";
  } else if (archived) {
    action = "Открыть архивный материал";
  }
  if ((draft || pending) && material.ownerUserId !== userId) {
    action = "Открыть рабочую версию";
  }
  let explanation =
    "Публичной публикации пока нет. Откройте материал, чтобы продолжить работу.";
  if (archived) {
    explanation =
      "Материал в архиве. Управление и история доступны в его рабочей области.";
  } else if (pending) {
    explanation = published
      ? "Новая версия ожидает решения модератора. Текущая публикация продолжает действовать."
      : "Материал ожидает решения модератора и пока не опубликован.";
  } else if (draft) {
    explanation = published
      ? "Текущая публикация продолжает действовать. Новая версия находится в работе."
      : "Черновик ещё не отправлен на модерацию.";
  } else if (published) {
    explanation =
      material.visibility === "public"
        ? "Материал опубликован. Доступ может зависеть от действующих ограничений."
        : "Опубликованная версия имеет ограниченный доступ.";
  }
  const owner =
    material.ownerUserId === userId ? "мой аккаунт" : material.ownerName;
  const canOpenPublication =
    published && material.visibility === "public" && !archived;
  let Icon = FileTextIcon;
  if (material.materialType === "book") {
    Icon = BookOpenIcon;
  } else if (
    material.materialType === "video" ||
    material.materialType === "lecture"
  ) {
    Icon = PlayIcon;
  } else if (material.materialType === "lab") {
    Icon = CodeIcon;
  }
  return (
    <article data-material-panel className={styles.card}>
      <div className={styles.materialIcon}>
        <Icon size={34} aria-hidden="true" />
      </div>
      <div className={styles.cardContent}>
        <h3>
          <Link to={workspace}>{material.title}</Link>
        </h3>
        <div className={styles.badges}>
          {material.materialType && (
            <span className={styles.kind}>
              {materialTypeLabels[material.materialType]}
            </span>
          )}
          {material.sourceYear !== null && (
            <span className={styles.kind}>{material.sourceYear}</span>
          )}
          {archived && (
            <PublicationStatus tone="archived">В архиве</PublicationStatus>
          )}
          {published && (
            <PublicationStatus tone="published">Опубликован</PublicationStatus>
          )}
          {draft && (
            <PublicationStatus tone="draft">
              {published ? "Есть черновик новой редакции" : "Черновик"}
            </PublicationStatus>
          )}
          {pending && (
            <PublicationStatus tone="pending">
              {published ? "Новая версия на проверке" : "На проверке"}
            </PublicationStatus>
          )}
          {!published && !draft && !pending && !archived && (
            <PublicationStatus tone="archived">
              Не опубликован
            </PublicationStatus>
          )}
        </div>
        <p className={styles.metadata}>
          Владелец: {owner}
          {material.updatedAt && (
            <span title={exactPublication(material.updatedAt)}>
              {" "}
              · Изменён {relativePublication(material.updatedAt)}
            </span>
          )}
          {published && material.publishedRevisionNumber !== null && (
            <span> · Публичная версия {material.publishedRevisionNumber}</span>
          )}
        </p>
        <div className={styles.cardBottom}>
          <div>
            {material.description && (
              <p className={styles.description}>{material.description}</p>
            )}
            {(!material.description ||
              archived ||
              (published && (draft || pending))) && (
              <p className={styles.explanation}>{explanation}</p>
            )}
          </div>
          <div className={styles.cardActions}>
            <Link className={styles.cardAction} to={workspace}>
              <span>{action}</span>
              <ArrowRightIcon size={18} aria-hidden="true" />
            </Link>
          </div>
        </div>
      </div>
      <details
        className={styles.materialMenu}
        onKeyDown={(event) => {
          if (event.key === "Escape") {
            event.currentTarget.open = false;
            event.currentTarget.querySelector<HTMLElement>("summary")?.focus();
          }
        }}
        onBlur={(event) => {
          if (!event.currentTarget.contains(event.relatedTarget)) {
            event.currentTarget.open = false;
          }
        }}
      >
        <summary aria-label={`Действия: ${material.title}`}>
          <DotsThreeIcon size={24} weight="bold" aria-hidden="true" />
        </summary>
        <nav aria-label={`Действия с материалом: ${material.title}`}>
          <Link to={workspace}>Рабочая область</Link>
          {canOpenPublication && (
            <Link to={linkTo(`/materials/${material.id}`)}>
              Текущая публикация
            </Link>
          )}
          {canOpenPublication && (
            <Link to={linkTo(`/materials/${material.id}/history`)}>
              История публикаций
            </Link>
          )}
        </nav>
      </details>
    </article>
  );
}
