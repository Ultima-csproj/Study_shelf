import { materialTypeLabels } from "../../../utils/materialLabels";
import type { MaterialType } from "../../../types/material";
import styles from "./Badge.module.css";

interface BadgeProps {
  type: MaterialType;
}

export function Badge({ type }: BadgeProps) {
  return (
    <span className={`${styles.badge} ${styles[type]}`}>
      {materialTypeLabels[type]}
    </span>
  );
}
