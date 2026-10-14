import type { ReactNode } from "react";
import styles from "./PublicationStatus.module.css";

export type PublicationTone =
  "draft" | "pending" | "published" | "archived" | "rejected";

export function PublicationStatus({
  tone,
  children,
}: {
  tone: PublicationTone;
  children: ReactNode;
}) {
  return <span className={`${styles.badge} ${styles[tone]}`}>{children}</span>;
}
