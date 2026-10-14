import styles from "./StatusBadge.module.css";

const labels: Record<string, string> = {
  active: "Подтверждено",
  approved: "Принято",
  accepted: "Принято",
  pending: "На рассмотрении",
  pending_moderation: "На проверке",
  rejected: "Отклонено",
  revoked: "Отозвано",
  published: "Опубликовано",
  draft: "Черновик",
};

export function StatusBadge({
  state,
  label,
}: {
  state: string;
  label?: string;
}) {
  let tone = "neutral";
  if (["active", "approved", "accepted", "published"].includes(state)) {
    tone = "success";
  } else if (["pending", "pending_moderation"].includes(state)) {
    tone = "warning";
  } else if (["rejected", "revoked", "off"].includes(state)) {
    tone = "danger";
  }
  return (
    <span className={`${styles.badge} ${styles[tone]}`}>
      {label ?? labels[state] ?? state}
    </span>
  );
}
