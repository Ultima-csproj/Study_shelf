import { useState } from "react";
import { EnvelopeSimpleIcon } from "@phosphor-icons/react/dist/csr/EnvelopeSimple";
import { ChatCircleIcon } from "@phosphor-icons/react/dist/csr/ChatCircle";
import { BellIcon } from "@phosphor-icons/react/dist/csr/Bell";
import { EyeIcon } from "@phosphor-icons/react/dist/csr/Eye";
import { BuildingsIcon } from "@phosphor-icons/react/dist/csr/Buildings";
import styles from "./SettingsPage.module.css";

interface SettingsPreferencesProps {
  kind: "notifications" | "privacy";
}

const notificationRows = [
  { key: "email", label: "Email-уведомления", Icon: EnvelopeSimpleIcon },
  { key: "comments", label: "Новые комментарии", Icon: ChatCircleIcon },
  { key: "updates", label: "Обновления материалов", Icon: BellIcon },
];
const privacyRows = [
  { key: "profile", label: "Публичный профиль", Icon: EyeIcon },
  { key: "institutions", label: "Показывать учреждения", Icon: BuildingsIcon },
  { key: "showEmail", label: "Показывать email", Icon: EnvelopeSimpleIcon },
];

export function SettingsPreferences({ kind }: SettingsPreferencesProps) {
  const [enabled, setEnabled] = useState<Record<string, boolean>>({
    email: true,
    comments: true,
    updates: false,
    profile: false,
    institutions: true,
    showEmail: false,
  });
  const rows = kind === "notifications" ? notificationRows : privacyRows;
  const title = kind === "notifications" ? "Уведомления" : "Приватность";

  return (
    <section className={styles.card}>
      <header className={styles.cardHeading}>
        <h2>{title}</h2>
        <span className={styles.previewBadge}>Скоро</span>
      </header>
      {rows.map(({ key, label, Icon }) => (
        <div className={styles.row} key={key}>
          <Icon size={21} aria-hidden="true" />
          <span>{label}</span>
          <button
            type="button"
            className={styles.switch}
            role="switch"
            aria-checked={enabled[key]}
            aria-label={`${label} (макет)`}
            onClick={() =>
              setEnabled((current) => ({ ...current, [key]: !current[key] }))
            }
          >
            <span />
          </button>
        </div>
      ))}
      <p className={styles.hint}>
        Будет добавлено в следующем обновлении. Пока это предпросмотр: изменения
        не сохраняются.
      </p>
    </section>
  );
}
