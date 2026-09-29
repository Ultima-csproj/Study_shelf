import { useEffect, useState } from "react";
import { getCatalog } from "../api/catalog";
import type {
  CatalogFilters,
  CatalogRequest,
  CatalogResponse,
} from "../types/material";

export const emptyFilters: CatalogFilters = {
  query: "",
  types: [],
  disciplineIds: [],
  teacherIds: [],
  courses: [],
  years: [],
  tags: [],
};

export function useCatalog(request: CatalogRequest) {
  const [result, setResult] = useState<CatalogResponse>({
    items: [],
    total: 0,
    page: 1,
    pageCount: 1,
    facets: {
      types: {},
      disciplines: [],
      teachers: [],
      courses: [],
      years: [],
    },
  });
  const [settled, setSettled] = useState<{
    request: CatalogRequest;
    attempt: number;
  } | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [attempt, setAttempt] = useState(0);
  useEffect(() => {
    const controller = new AbortController();
    getCatalog(request, controller.signal)
      .then((response) => {
        if (!controller.signal.aborted) {
          setResult(response);
          setError(null);
          setSettled({ request, attempt });
        }
      })
      .catch((reason: unknown) => {
        if (!controller.signal.aborted) {
          setError(
            reason instanceof Error
              ? reason.message
              : "Не удалось загрузить материалы",
          );
          setSettled({ request, attempt });
        }
      });
    return () => controller.abort();
  }, [request, attempt]);
  const loading = settled?.request !== request || settled.attempt !== attempt;
  return {
    result,
    loading,
    error: loading ? null : error,
    retry: () => setAttempt((value) => value + 1),
  };
}
