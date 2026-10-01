import { materialTypeLabels } from "../../../utils/materialLabels";
import type { MaterialType } from "../../../types/material";
import styles from "./Badge.module.css";

export function Badge({ type }: { type: MaterialType }) {
  return (
    <span className={`${styles.badge} ${styles[type]}`}>
      {materialTypeLabels[type]}
    </span>
  );
}
