import { useNavigate } from "react-router-dom";
import { Select } from "../../common/Select/Select";
import type { DisciplineSummary } from "../../../types/discipline";
import styles from "./DisciplineSelector.module.css";
export function DisciplineSelector({
  disciplines,
  value,
}: {
  disciplines: DisciplineSummary[];
  value: string;
}) {
  const navigate = useNavigate();
  const options = disciplines.map((discipline) => ({
    value: discipline.slug,
    label: discipline.name,
  }));
  return (
    <div className={styles.selector}>
      <span aria-hidden="true">Дисциплина</span>
      <Select
        label="Дисциплина"
        value={value}
        onChange={(slug) => navigate(`/disciplines/${slug}`)}
        options={options}
      />
    </div>
  );
}
