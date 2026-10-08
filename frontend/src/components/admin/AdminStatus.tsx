import { HttpError } from "../../api/http";
import { WarningCircleIcon } from "@phosphor-icons/react/dist/csr/WarningCircle";
import styles from "./Admin.module.css";

interface AdminStatusProps {
  message: string;
  failure?: unknown;
  retry?: () => void;
}

export function AdminStatus({ message, failure, retry }: AdminStatusProps) {
  let title = "Не удалось загрузить данные";
  let detail = message;
  if (failure instanceof HttpError) {
    if (failure.status === 401) {
      title = "Нужно войти снова";
      detail = "Сессия завершилась. Войдите в аккаунт и обновите страницу.";
    } else if (failure.status === 403) {
      title = "Недостаточно полномочий";
      detail =
        "Этот раздел доступен сотрудникам с соответствующим разрешением.";
    } else if (failure.status >= 500) {
      title = "Сервер не смог загрузить данные";
      detail =
        "Попробуйте ещё раз. Если ошибка повторится, сообщите администратору платформы.";
    }
  } else if (message.includes("связаться с сервером")) {
    title = "Нет связи с сервером";
    detail = "Проверьте подключение и повторите загрузку.";
  }
  return (
    <div className={styles.status} role="alert">
      <WarningCircleIcon size={22} aria-hidden="true" />
      <div>
        <strong>{title}</strong>
        <p>{detail}</p>
        {retry && (
          <button type="button" onClick={retry}>
            Повторить загрузку
          </button>
        )}
      </div>
    </div>
  );
}
