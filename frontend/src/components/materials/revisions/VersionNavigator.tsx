import type { Page } from "../../../api/library";
import {
  revisionLabels,
  type RevisionHeader,
} from "../../../api/materialWorkspace";
import { Button } from "../../common/Button/Button";
import { Pagination } from "../../common/Pagination/Pagination";
import styles from "./VersionNavigator.module.css";

interface VersionNavigatorProps {
  revisions: Page<RevisionHeader>;
  selected: number;
  page: number;
  loading: boolean;
  onChoose: (id: number) => void;
  onPage: (page: number) => void;
}

export function VersionNavigator({
  revisions,
  selected,
  page,
  loading,
  onChoose,
  onPage,
}: VersionNavigatorProps) {
  return (
    <div>
      {revisions.total === 0 && <p>Версий пока нет.</p>}
      <ul className={styles.list}>
        {revisions.items.map((revision) => (
          <li key={revision.id}>
            <Button
              variant="secondary"
              className={styles.version}
              aria-pressed={selected === revision.id}
              disabled={loading}
              onClick={() => onChoose(revision.id)}
            >
              <span className={styles.heading}>
                <span>Версия {revision.number}</span>
                <span className={styles.status}>
                  {revisionLabels[revision.state] ?? revision.state}
                </span>
              </span>
              <span className={styles.title}>{revision.title}</span>
            </Button>
          </li>
        ))}
      </ul>
      <Pagination
        page={page}
        pageCount={revisions.pageCount}
        onChange={onPage}
        loading={loading}
        label="Страницы редакций"
      />
    </div>
  );
}
