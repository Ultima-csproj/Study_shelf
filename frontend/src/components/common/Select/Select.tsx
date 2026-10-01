import { CaretDownIcon } from "@phosphor-icons/react/dist/csr/CaretDown";
import styles from "./Select.module.css";

interface SelectProps {
  value: string;
  label: string;
  onChange: (value: string) => void;
  options: { value: string; label: string }[];
}
export function Select({ value, label, onChange, options }: SelectProps) {
  return (
    <label className={styles.select}>
      <span className="visually-hidden">{label}</span>
      <select value={value} onChange={(event) => onChange(event.target.value)}>
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
