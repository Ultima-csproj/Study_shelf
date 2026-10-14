import styles from "./Admin.module.css";
import { Pagination } from "../common/Pagination/Pagination";

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
    <div className={styles.pagination}>
      <span>Всего: {total}</span>
      <Pagination page={page} pageCount={pageCount} onChange={onPage} />
    </div>
  );
}
