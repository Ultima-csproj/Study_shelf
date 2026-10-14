import {
  revisionLabels,
  creditLabels,
  type RevisionResponse,
} from "../../../api/materialWorkspace";
import styles from "../shared/Panel.module.css";

interface RevisionContentProps {
  value: RevisionResponse;
}
export function RevisionContent({ value }: RevisionContentProps) {
  const revision = value.revision;
  return (
    <article data-material-panel className={styles.card}>
      <h2>{revision.title}</h2>
      <p>
        Версия {revision.number} ·{" "}
        {revisionLabels[revision.state] ?? revision.state}
      </p>
      <details>
        <summary>Технические сведения</summary>
        <p>
          ID версии: #{revision.id}.{" "}
          {revision.baseRevisionId === null
            ? "Первая публикация."
            : "ID базовой версии: #" + revision.baseRevisionId}
        </p>
      </details>
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
          Решение модератора:{" "}
          {value.decision.decision === "approved" ? "принято" : "отклонено"}.{" "}
          {value.decision.reason}
          {value.decision.wasSelfReview &&
            " · Проверяющий участвовал в подготовке этой редакции."}
        </p>
      )}
    </article>
  );
}
