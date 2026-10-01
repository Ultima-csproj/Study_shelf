import { CaretLeftIcon } from "@phosphor-icons/react/dist/csr/CaretLeft";
import { CaretRightIcon } from "@phosphor-icons/react/dist/csr/CaretRight";
import styles from "./Pagination.module.css";

interface PaginationProps {
  page: number;
  pageCount: number;
  total: number;
  pageSize: number;
  onChange: (page: number) => void;
}
function pageNumbers(
  page: number,
  count: number,
): (number | "gap-left" | "gap-right")[] {
  if (count <= 7) {
    return Array.from({ length: count }, (_, index) => index + 1);
  }
  const start = Math.max(2, page - 1);
  const end = Math.min(count - 1, page + 1);
  return [
    1,
    ...(start > 2 ? ["gap-left" as const] : []),
    ...Array.from({ length: end - start + 1 }, (_, index) => start + index),
    ...(end < count - 1 ? ["gap-right" as const] : []),
    count,
  ];
}

export function Pagination({
  page,
  pageCount,
  total,
  pageSize,
  onChange,
}: PaginationProps) {
  if (!total) {
    return null;
  }
  return (
    <div className={styles.footer}>
      <p>
        Показаны {(page - 1) * pageSize + 1}–{Math.min(page * pageSize, total)}{" "}
        из {total} материалов
      </p>
      <nav aria-label="Страницы каталога" className={styles.pages}>
        <button
          type="button"
          onClick={() => onChange(page - 1)}
          disabled={page === 1}
          aria-label="Предыдущая страница"
        >
          <CaretLeftIcon size={14} aria-hidden="true" />
        </button>
        {pageNumbers(page, pageCount).map((number) =>
          typeof number === "number" ? (
            <button
              type="button"
              key={number}
              onClick={() => onChange(number)}
              aria-label={`Страница ${number}`}
              aria-current={number === page ? "page" : undefined}
            >
              {number}
            </button>
          ) : (
            <span key={number} aria-hidden="true">
              …
            </span>
          ),
        )}
        <button
          type="button"
          onClick={() => onChange(page + 1)}
          disabled={page === pageCount}
          aria-label="Следующая страница"
        >
          <CaretRightIcon size={14} aria-hidden="true" />
        </button>
      </nav>
    </div>
  );
}
