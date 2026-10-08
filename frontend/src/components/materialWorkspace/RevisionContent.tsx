import {
  revisionLabels,
  creditLabels,
  type RevisionResponse,
} from "../../api/materialWorkspace";
import styles from "./Workspace.module.css";

interface RevisionContentProps {
  value: RevisionResponse;
}
export function RevisionContent({ value }: RevisionContentProps) {
  const revision = value.revision;
  return (
    <article className={styles.card}>
      <h2>{revision.title}</h2>
      <p>
        Редакция #{revision.id} ·{" "}
        {revisionLabels[revision.state] ?? revision.state}
      </p>
      <p>
        База:{" "}
        {revision.baseRevisionId === null
          ? "первая публикация"
          : "#" + revision.baseRevisionId}
      </p>
      {value.outdated && (
        <p className={styles.banner}>
          Опубликованная база изменилась. Перед отправкой или принятием
          пересмотрите черновик на актуальной базе.
        </p>
      )}
      <p className={styles.body}>{revision.description}</p>
      <ul className={styles.list}>
        {revision.credits.map((credit) => (
          <li key={credit.contributorId + ":" + credit.role}>
            {creditLabels[credit.role]}: {credit.displayName}
          </li>
        ))}
      </ul>
      {value.decision && (
        <p>
          Решение:{" "}
          {value.decision.decision === "approved" ? "принято" : "отклонено"}.{" "}
          {value.decision.reason}
          {value.decision.wasSelfReview &&
            " · Проверяющий участвовал в подготовке этой редакции."}
        </p>
      )}
    </article>
  );
}
