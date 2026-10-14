import { useCallback, useState, type ReactNode, type FormEvent } from "react";
import { useSearchParams } from "react-router-dom";
import type { Page } from "../../api/library";
import type { AdminQuery } from "../../api/admin";
import { useApiResource } from "../../hooks/useApiResource";
import { useViewer } from "../../hooks/useViewer";
import { AdminStatus } from "./AdminStatus";
import { AdminPagination } from "./AdminPagination";
import styles from "./Admin.module.css";

export type RunAction = (operation: () => Promise<void>) => Promise<boolean>;
export interface AdminColumn<T> {
  title: string;
  cell: (item: T) => ReactNode;
}
interface AdminListProps<T> {
  title: string;
  load: (query: AdminQuery, signal: AbortSignal) => Promise<Page<T>>;
  render: (item: T, run: RunAction) => ReactNode;
  states?: string[];
  stateLabels?: Record<string, string>;
  onChanged?: () => void;
  heading?: "h1" | "h2";
  columns?: AdminColumn<T>[];
  emptyMessage?: string;
}
export function AdminList<T>({
  title,
  load,
  render,
  states = [],
  stateLabels = {},
  onChanged,
  heading = "h1",
  columns,
  emptyMessage = "Здесь пока нет записей.",
}: AdminListProps<T>) {
  const [params, setParams] = useSearchParams();
  const q = params.get("q") ?? "";
  const state = params.get("state") ?? "";
  const requested = Number(params.get("page") ?? "1");
  const page = Number.isSafeInteger(requested) && requested > 0 ? requested : 1;
  const table = columns !== undefined && params.get("view") === "table";
  const [search, setSearch] = useState({ query: q, text: q });
  const searchText = search.query === q ? search.text : q;
  const [revision, setRevision] = useState(0);
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const { refreshViewer } = useViewer();
  const Heading = heading;
  const fetchPage = useCallback(
    (signal: AbortSignal) => {
      void revision;
      return load({ page, q, state }, signal);
    },
    [load, page, q, state, revision],
  );
  const { result, loading, error, failure, retry } = useApiResource(
    fetchPage,
    "Не удалось загрузить данные",
  );
  const updateParams = (values: Record<string, string>) => {
    const next = new URLSearchParams(params);
    for (const [key, value] of Object.entries(values)) {
      if (value) {
        next.set(key, value);
      } else {
        next.delete(key);
      }
    }
    setParams(next);
  };
  const submit = (event: FormEvent) => {
    event.preventDefault();
    updateParams({ q: searchText, page: "1" });
  };
  const run: RunAction = async (operation) => {
    if (busy) {
      return false;
    }
    setBusy(true);
    setNotice(null);
    try {
      await operation();
      setNotice("Изменения сохранены.");
      setRevision((value) => value + 1);
      onChanged?.();
      await refreshViewer();
      return true;
    } catch (reason) {
      setNotice(
        reason instanceof Error
          ? reason.message
          : "Не удалось выполнить действие",
      );
      return false;
    } finally {
      setBusy(false);
    }
  };
  return (
    <section className={styles.list}>
      <div className={styles.listHeading}>
        <Heading>{title}</Heading>
        {columns && (
          <div
            className={styles.viewSwitch}
            role="group"
            aria-label="Вид списка"
          >
            <button
              type="button"
              aria-pressed={!table}
              onClick={() => updateParams({ view: "" })}
            >
              Карточки
            </button>
            <button
              type="button"
              aria-pressed={table}
              onClick={() => updateParams({ view: "table" })}
            >
              Таблица
            </button>
          </div>
        )}
      </div>
      <form className={styles.filters} onSubmit={submit}>
        <label>
          Поиск
          <input
            maxLength={200}
            placeholder="Название или номер"
            value={searchText}
            onChange={(event) =>
              setSearch({ query: q, text: event.target.value })
            }
          />
        </label>
        {states.length > 0 && (
          <label>
            Статус
            <select
              value={state}
              onChange={(event) =>
                updateParams({ state: event.target.value, page: "1" })
              }
            >
              <option value="">Все</option>
              {states.map((value) => (
                <option key={value} value={value}>
                  {stateLabels[value] ?? value}
                </option>
              ))}
            </select>
          </label>
        )}
        <button type="submit">Найти</button>
        {(q || state) && (
          <button
            type="button"
            onClick={() => updateParams({ q: "", state: "", page: "1" })}
          >
            Сбросить
          </button>
        )}
      </form>
      {loading && (
        <p className={styles.loading} role="status">
          Загружаем список…
        </p>
      )}
      {error && <AdminStatus message={error} failure={failure} retry={retry} />}
      {notice && (
        <p className={styles.notice} role="status">
          {notice}{" "}
          <button type="button" onClick={retry}>
            Обновить данные
          </button>
        </p>
      )}
      <fieldset disabled={busy || loading} className={styles.fieldset}>
        {result?.items.length === 0 && (
          <div className={styles.empty}>
            <h2>{q || state ? "Ничего не найдено" : emptyMessage}</h2>
            <p>
              {q || state
                ? "Попробуйте другой запрос или сбросьте фильтры."
                : "Новые записи появятся здесь, когда будут доступны."}
            </p>
          </div>
        )}
        {result &&
          result.items.length > 0 &&
          (table && columns ? (
            <div className={styles.tableScroll} role="region" aria-label="Таблица с горизонтальной прокруткой" tabIndex={0}>
              <table className={styles.table}>
                <caption className="visually-hidden">{title}</caption>
                <thead>
                  <tr>
                    {columns.map((column) => (
                      <th key={column.title} scope="col">
                        {column.title}
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {result.items.map((item, index) => (
                    <tr key={index}>
                      {columns.map((column) => (
                        <td key={column.title}>{column.cell(item)}</td>
                      ))}
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          ) : (
            <div className={styles.cards}>
              {result.items.map((item, index) => (
                <div key={index}>{render(item, run)}</div>
              ))}
            </div>
          ))}
      </fieldset>
      {result && (
        <AdminPagination
          page={page}
          pageCount={result.pageCount}
          total={result.total}
          onPage={(value) => updateParams({ page: String(value) })}
        />
      )}
    </section>
  );
}
