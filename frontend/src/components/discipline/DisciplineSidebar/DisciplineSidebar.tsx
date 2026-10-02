import { Link } from "react-router-dom";
import type { DisciplineSummary } from "../../../types/discipline";
import styles from "./DisciplineSidebar.module.css";

interface DisciplineSidebarProps {
  disciplines: DisciplineSummary[];
  selectedId: number;
}

export function DisciplineSidebar({
  disciplines,
  selectedId,
}: DisciplineSidebarProps) {
  return (
    <aside className={styles.sidebar}>
      <nav aria-label="Дисциплины">
        <h2>Дисциплины</h2>
        {disciplines.map((discipline) => {
          const selected = discipline.id === selectedId;
          let className = "";
          let currentPage: "page" | undefined = undefined;

          if (selected) {
            className = styles.selected;
            currentPage = "page";
          }

          return (
            <Link
              key={discipline.id}
              to={`/disciplines/${discipline.slug}`}
              className={className}
              aria-current={currentPage}
            >
              <span>{discipline.name}</span>
              <span className={styles.count}>{discipline.materialCount}</span>
            </Link>
          );
        })}
      </nav>
    </aside>
  );
}
