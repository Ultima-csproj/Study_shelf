import type { ChangeEventHandler } from "react";
import { CheckIcon } from "@phosphor-icons/react/dist/csr/Check";
import styles from "./Checkbox.module.css";
import { materialCount } from "../../../utils/format";

interface CheckboxProps {
  label: string;
  count: number;
  checked: boolean;
  onChange: ChangeEventHandler<HTMLInputElement>;
}
export function Checkbox({ label, count, checked, onChange }: CheckboxProps) {
  return (
    <label className={styles.checkbox}>
      <span className={styles.control}>
        <input type="checkbox" checked={checked} onChange={onChange} />
        <CheckIcon size={14} weight="bold" aria-hidden="true" />
      </span>
      <span className={styles.label}>{label}</span>
      <span className={styles.count} aria-label={materialCount(count)}>
        {count}
      </span>
    </label>
  );
}
