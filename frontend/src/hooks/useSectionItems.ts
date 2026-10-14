import { useEffect, useRef, useState } from "react";
import type { ApiPage } from "../types/learning";

export function useSectionItems<T extends { id: number }>(
  initial: T[],
  total: number,
  load: (page: number, signal: AbortSignal) => Promise<ApiPage<T>>,
) {
  const controller = useRef<AbortController | null>(null);
  const [state, setState] = useState({
    initial,
    items: initial,
    total,
    page: 1,
    loading: false,
    error: null as string | null,
    finished: false,
  });
  useEffect(() => {
    return () => controller.current?.abort();
  }, [initial]);
  let current = state;
  if (state.initial !== initial) {
    current = { initial, items: initial, total, page: 1, loading: false, error: null, finished: false };
  }
  const loadMore = async () => {
    if (current.loading) {
      return;
    }
    controller.current?.abort();
    const request = new AbortController();
    controller.current = request;
    setState({ ...current, loading: true, error: null });
    try {
      const response = await load(current.page + 1, request.signal);
      if (request.signal.aborted) {
        return;
      }
      const ids = new Set(current.items.map((item) => item.id));
      const added = response.items.filter((item) => !ids.has(item.id));
      setState({
        initial,
        items: [...current.items, ...added],
        total: response.total,
        page: response.page,
        loading: false,
        error: null,
        finished: response.page >= response.pageCount,
      });
    } catch (reason) {
      if (!request.signal.aborted) {
        setState({
          ...current,
          loading: false,
          error: reason instanceof Error ? reason.message : "Не удалось загрузить продолжение",
        });
      }
    }
  };
  return {
    items: current.items,
    loading: current.loading,
    error: current.error,
    hasMore: !current.finished && current.items.length < current.total,
    loadMore,
  };
}
