import { useCallback, useState, type ReactNode } from "react";
import { useApiResource } from "../../hooks/useApiResource";
import type { Page } from "../../api/library";
import styles from "./Workspace.module.css";

interface PagedPanelProps<T> {
  title: string;
  load: (page: number, signal: AbortSignal) => Promise<Page<T>>;
  render: (item: T) => ReactNode;
}
export function PagedPanel<T>({ title, load, render }: PagedPanelProps<T>) {
  const [page, setPage] = useState(1);
  const fetch = useCallback(
    (signal: AbortSignal) => load(page, signal),
    [load, page],
  );
  const { result, loading, error, retry } = useApiResource(
    fetch,
    "Не удалось загрузить список",
  );
  return (
    <section className={styles.card}>
      <h2>{title}</h2>
      {loading && <p role="status">Загружаем…</p>}
      {error && (
        <p role="alert">
          {error}{" "}
          <button type="button" onClick={retry}>
            Повторить
          </button>
        </p>
      )}
      {result?.items.length === 0 && <p>Пока нет записей.</p>}
      {result?.items.map((item, index) => (
        <div key={index}>{render(item)}</div>
      ))}
      {result && (
        <nav className={styles.actions} aria-label={"Страницы: " + title}>
          <button
            type="button"
            disabled={page <= 1}
            onClick={() => setPage((value) => value - 1)}
          >
            Назад
          </button>
          <span>
            {page} / {Math.max(1, result.pageCount)}
          </span>
          <button
            type="button"
            disabled={page >= result.pageCount}
            onClick={() => setPage((value) => value + 1)}
          >
            Далее
          </button>
        </nav>
      )}
    </section>
  );
}
