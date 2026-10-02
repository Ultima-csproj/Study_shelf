import { useState, type ReactNode, type SyntheticEvent } from "react";
import { CaretUpIcon } from "@phosphor-icons/react/dist/csr/CaretUp";
import styles from "./CatalogFilters.module.css";

interface FilterGroupProps {
  title: string;
  children: ReactNode;
}

export function FilterGroup({ title, children }: FilterGroupProps) {
  const [open, setOpen] = useState(true);

  const handleToggle = (event: SyntheticEvent<HTMLDetailsElement>) => {
    const isOpen = event.currentTarget.open;
    setOpen(isOpen);
  };

  return (
    <details
      open={open}
      onToggle={handleToggle}
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
