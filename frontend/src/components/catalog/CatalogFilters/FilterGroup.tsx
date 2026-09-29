import { useState, type ReactNode } from "react";
import { CaretUpIcon } from "@phosphor-icons/react/dist/csr/CaretUp";
import styles from "./CatalogFilters.module.css";

export function FilterGroup({
  title,
  children,
}: {
  title: string;
  children: ReactNode;
}) {
  const [open, setOpen] = useState(true);
  return (
    <details
      open={open}
      onToggle={(event) => setOpen(event.currentTarget.open)}
      className={styles.group}
    >
      <summary>
        <span>{title}</span>
        <CaretUpIcon size={13} aria-hidden="true" />
      </summary>
      <div className={styles.options}>{children}</div>
    </details>
  );
}
