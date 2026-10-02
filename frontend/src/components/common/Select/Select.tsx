import type { ChangeEvent } from "react";
import { CaretDownIcon } from "@phosphor-icons/react/dist/csr/CaretDown";
import styles from "./Select.module.css";

interface SelectOption {
  value: string;
  label: string;
}

interface SelectProps {
  value: string;
  label: string;
  onChange: (value: string) => void;
  options: SelectOption[];
}
export function Select({ value, label, onChange, options }: SelectProps) {
  const handleChange = (event: ChangeEvent<HTMLSelectElement>) => {
    const selectedValue = event.target.value;
    onChange(selectedValue);
  };

  return (
    <label className={styles.select}>
      <span className="visually-hidden">{label}</span>
      <select value={value} onChange={handleChange}>
        {options.map((option) => (
          <option key={option.value} value={option.value}>
            {option.label}
          </option>
        ))}
      </select>
      <CaretDownIcon size={14} aria-hidden="true" />
    </label>
  );
}
