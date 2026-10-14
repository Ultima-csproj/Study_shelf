import { CaretLeftIcon } from "@phosphor-icons/react/dist/csr/CaretLeft";
import { CaretRightIcon } from "@phosphor-icons/react/dist/csr/CaretRight";
import { Button } from "../Button/Button";
import styles from "./Pagination.module.css";

interface PaginationProps {
  page: number;
  pageCount: number;
  onChange: (page: number) => void;
  label?: string;
  loading?: boolean;
}

export function Pagination({
  page,
  pageCount,
  onChange,
  label = "Страницы списка",
  loading = false,
}: PaginationProps) {
  if (pageCount <= 1 && page <= 1) {
    return null;
  }

  return (
    <nav className={styles.navigation} aria-label={label} aria-busy={loading}>
      <Button
        variant="secondary"
        size="compact"
        disabled={loading || page <= 1}
        onClick={() => onChange(Math.max(1, Math.min(page - 1, pageCount)))}
      >
        <CaretLeftIcon size={16} aria-hidden="true" />
        Назад
      </Button>
      <span className={styles.counter} aria-live="polite" aria-atomic="true">
        {page} из {Math.max(1, pageCount)}
      </span>
      <Button
        variant="secondary"
        size="compact"
        disabled={loading || page >= pageCount}
        onClick={() => onChange(page + 1)}
      >
        Далее
        <CaretRightIcon size={16} aria-hidden="true" />
      </Button>
    </nav>
  );
}
