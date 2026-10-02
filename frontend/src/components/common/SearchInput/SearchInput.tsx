import { useId, type ChangeEvent, type ReactNode } from "react";
import { MagnifyingGlassIcon } from "@phosphor-icons/react/dist/csr/MagnifyingGlass";
import { XIcon } from "@phosphor-icons/react/dist/csr/X";
import styles from "./SearchInput.module.css";

interface SearchInputProps {
  value: string;
  onChange: (value: string) => void;
  label: string;
  placeholder: string;
  compact?: boolean;
}
export function SearchInput({
  value,
  onChange,
  label,
  placeholder,
  compact = false,
}: SearchInputProps) {
  const id = useId();
  let className = styles.field;

  if (compact) {
    className = `${styles.field} ${styles.compact}`;
  }

  const handleClear = () => {
    onChange("");
  };

  const handleInputChange = (event: ChangeEvent<HTMLInputElement>) => {
    const nextValue = event.target.value;
    onChange(nextValue);
  };

  let clearButton: ReactNode = null;

  if (value) {
    clearButton = (
      <button
        type="button"
        className={styles.clear}
        onClick={handleClear}
        aria-label={`Очистить: ${label}`}
      >
        <XIcon size={16} aria-hidden="true" />
      </button>
    );
  }

  return (
    <div className={className}>
      <label className="visually-hidden" htmlFor={id}>
        {label}
      </label>
      <MagnifyingGlassIcon
        size={19}
        aria-hidden="true"
        className={styles.icon}
      />
      <input
        id={id}
        type="search"
        value={value}
        onChange={handleInputChange}
        placeholder={placeholder}
        autoComplete="off"
      />
      {clearButton}
    </div>
  );
}
