import { useEffect, useState } from "react";
import {
  getDiscipline,
  getDisciplineMaterials,
  getDisciplines,
} from "../api/disciplines";
import type { DisciplineSummary } from "../types/discipline";
import type { MaterialSectionData } from "../types/material";
interface DisciplineResult {
  discipline: DisciplineSummary | null;
  navigation: DisciplineSummary[];
  sections: MaterialSectionData[];
}
export function useDiscipline(id: string) {
  const [attempt, setAttempt] = useState(0);
  const key = `${id}:${attempt}`;
  const [state, setState] = useState<{
    key: string;
    result: DisciplineResult | null;
    error: string | null;
  }>({ key: "", result: null, error: null });
  useEffect(() => {
    const controller = new AbortController();
    Promise.all([
      getDiscipline(id, controller.signal),
      getDisciplines(controller.signal),
      getDisciplineMaterials(id, controller.signal),
    ])
      .then(([discipline, navigation, sections]) => {
        if (!controller.signal.aborted) {
          setState({
            key,
            result: { discipline, navigation, sections },
            error: null,
          });
        }
      })
      .catch((reason: unknown) => {
        if (!controller.signal.aborted) {
          setState({
            key,
            result: null,
            error:
              reason instanceof Error
                ? reason.message
                : "Не удалось загрузить дисциплину",
          });
        }
      });
    return () => controller.abort();
  }, [id, key]);
  return {
    result: state.key === key ? state.result : null,
    error: state.key === key ? state.error : null,
    loading: state.key !== key,
    retry: () => setAttempt((value) => value + 1),
  };
}
