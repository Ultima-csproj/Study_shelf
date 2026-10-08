import styles from "./Admin.module.css";

interface AdminPaginationProps {
  page: number;
  pageCount: number;
  total: number;
  onPage: (page: number) => void;
}

export function AdminPagination({
  page,
  pageCount,
  total,
  onPage,
}: AdminPaginationProps) {
  if (total === 0) {
    return null;
  }
  return (
    <nav className={styles.pagination} aria-label="Страницы списка">
      <span>Всего: {total}</span>
      {pageCount > 1 && (
        <div className={styles.actions}>
          <button
            type="button"
            disabled={page <= 1}
            onClick={() => onPage(page - 1)}
          >
            Назад
          </button>
          <span>
            Страница {page} из {pageCount}
          </span>
          <button
            type="button"
            disabled={page >= pageCount}
            onClick={() => onPage(page + 1)}
          >
            Далее
          </button>
        </div>
      )}
    </nav>
  );
}
