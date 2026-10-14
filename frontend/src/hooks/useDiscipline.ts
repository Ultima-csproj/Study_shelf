import { useCallback } from "react";
import { getDiscipline, getDisciplines } from "../api/disciplines";
import { useApiResource } from "./useApiResource";

export function useDiscipline(id: string) {
  const load = useCallback(async (signal: AbortSignal) => {
    const [detail, navigation] = await Promise.all([
      getDiscipline(id, signal),
      getDisciplines(signal),
    ]);
    if (detail && !navigation.items.some((item) => item.id === detail.discipline.id)) {
      navigation.items.push(detail.discipline);
    }
    return {
      discipline: detail?.discipline ?? null,
      sections: detail?.sections ?? [],
      navigation: navigation.items,
    };
  }, [id]);
  return useApiResource(load, "Не удалось загрузить дисциплину");
}
