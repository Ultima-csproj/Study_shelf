import { useNavigate } from "react-router-dom";
import { Select } from "../../common/Select/Select";
import type { DisciplineSummary } from "../../../types/discipline";
import styles from "./DisciplineSelector.module.css";

interface DisciplineSelectorProps {
  disciplines: DisciplineSummary[];
  value: string;
}

export function DisciplineSelector({
  disciplines,
  value,
}: DisciplineSelectorProps) {
  const navigate = useNavigate();
  const options = disciplines.map((discipline) => {
    return {
      value: discipline.slug,
      label: discipline.name,
    };
  });

  const handleDisciplineChange = (slug: string) => {
    const path = `/disciplines/${slug}`;
    navigate(path);
  };

  return (
    <div className={styles.selector}>
      <span aria-hidden="true">Дисциплина</span>
      <Select
        label="Дисциплина"
        value={value}
        onChange={handleDisciplineChange}
        options={options}
      />
    </div>
  );
}
