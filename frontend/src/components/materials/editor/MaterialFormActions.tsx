import { useNavigationContext } from "../../../hooks/useNavigationContext";
import { Link } from "react-router-dom";
import type { ContentFormMode } from "./types";
import styles from "./ContentForm.module.css";
interface MaterialFormActionsProps {
  mode: ContentFormMode;
  busy: boolean;
  allowSubmit: boolean;
}
export function MaterialFormActions({
  mode,
  busy,
  allowSubmit,
}: MaterialFormActionsProps) {
  const { back } = useNavigationContext("/author/materials");
  return (
    <div className={styles.actions}>
      <button
        type="submit"
        value="save"
        className={
          mode === "create" || !allowSubmit ? styles.primary : undefined
        }
      >
        {busy
          ? "Сохраняем…"
          : mode === "create"
            ? "Создать черновик"
            : "Сохранить изменения"}
      </button>
      {allowSubmit && (
        <button type="submit" value="submit" className={styles.primary}>
          Отправить на модерацию
        </button>
      )}
      {mode === "create" && (
        <Link
          to={back.path}
          className={styles.cancel}
          aria-disabled={busy}
          onClick={(event) => {
            if (busy) {
              event.preventDefault();
            }
          }}
        >
          Отмена
        </Link>
      )}
      {mode === "create" && (
        <small>Отправить на модерацию можно будет после создания.</small>
      )}
      {allowSubmit && <small>Перед отправкой изменения будут сохранены.</small>}
    </div>
  );
}
