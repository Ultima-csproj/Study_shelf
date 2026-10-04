import { CaretLeftIcon } from "@phosphor-icons/react/dist/csr/CaretLeft";
import { CaretRightIcon } from "@phosphor-icons/react/dist/csr/CaretRight";
import type { ReactNode } from "react";
import styles from "./Pagination.module.css";

type PageNumber = number | "gap-left" | "gap-right";

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
): PageNumber[] {
  if (count <= 7) {
    return Array.from({ length: count }, (_, index) => index + 1);
  }

  const start = Math.max(2, page - 1);
  const end = Math.min(count - 1, page + 1);
  const visiblePages: PageNumber[] = [1];

  if (start > 2) {
    visiblePages.push("gap-left");
  }

  for (let currentPage = start; currentPage <= end; currentPage += 1) {
    visiblePages.push(currentPage);
  }

  if (end < count - 1) {
    visiblePages.push("gap-right");
  }

  visiblePages.push(count);

  return visiblePages;
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

  const firstVisibleItem = Math.min((page - 1) * pageSize + 1, total);
  const lastVisibleItem = Math.min(page * pageSize, total);
  let summary = `Показаны ${firstVisibleItem}–${lastVisibleItem} из ${total} материалов`;
  if (page > pageCount) {
    summary = `На странице ${page} материалов нет. Всего: ${total}`;
  }
  const visiblePages = pageNumbers(page, pageCount);
  const pageItems: ReactNode[] = [];

  for (const visiblePage of visiblePages) {
    if (typeof visiblePage === "number") {
      let ariaCurrent: "page" | undefined = undefined;

      if (visiblePage === page) {
        ariaCurrent = "page";
      }

      const pageButton = (
        <button
          type="button"
          key={visiblePage}
          onClick={() => onChange(visiblePage)}
          aria-label={`Страница ${visiblePage}`}
          aria-current={ariaCurrent}
        >
          {visiblePage}
        </button>
      );

      pageItems.push(pageButton);
    } else {
      const gap = (
        <span key={visiblePage} aria-hidden="true">
          …
        </span>
      );

      pageItems.push(gap);
    }
  }

  return (
    <div className={styles.footer}>
      <p>
        {summary}
      </p>
      <nav aria-label="Страницы каталога" className={styles.pages}>
        <button
          type="button"
          onClick={() => onChange(Math.min(page - 1, pageCount))}
          disabled={page === 1}
          aria-label="Предыдущая страница"
        >
          <CaretLeftIcon size={14} aria-hidden="true" />
        </button>
        {pageItems}
        <button
          type="button"
          onClick={() => onChange(page + 1)}
          disabled={page >= pageCount}
          aria-label="Следующая страница"
        >
          <CaretRightIcon size={14} aria-hidden="true" />
        </button>
      </nav>
    </div>
  );
}
