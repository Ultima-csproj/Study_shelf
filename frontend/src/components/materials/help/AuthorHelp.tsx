import { LockKeyIcon } from "@phosphor-icons/react/dist/csr/LockKey";
import { ClipboardTextIcon } from "@phosphor-icons/react/dist/csr/ClipboardText";
import { UsersIcon } from "@phosphor-icons/react/dist/csr/Users";
import styles from "./AuthorHelp.module.css";

export function AuthorHelp({ creating = false }: { creating?: boolean }) {
  return (
    <aside className={styles.help}>
      <h2>{creating ? "Полезно знать" : "Как проходит публикация"}</h2>
      <div className={styles.item}>
        <LockKeyIcon size={24} aria-hidden="true" />
        <div>
          <h3>Начните с черновика</h3>
          <p>
            Сохраните материал и возвращайтесь к нему позже. До публикации он
            доступен владельцу и приглашённым участникам.
          </p>
        </div>
      </div>
      <div className={styles.item}>
        <ClipboardTextIcon size={24} aria-hidden="true" />
        <div>
          <h3>Готовый материал отправьте на модерацию</h3>
          <p>
            Он станет публичным после одобрения. Отправка на проверку ещё не
            означает публикацию.
          </p>
        </div>
      </div>
      <div className={styles.item}>
        <UsersIcon size={24} aria-hidden="true" />
        <div>
          <h3>
            {creating
              ? "Авторство и доступ задаются отдельно"
              : "Новая версия не заменяет публикацию сразу"}
          </h3>
          <p>
            {creating
              ? "Укажите авторов здесь. Приглашения и права редактирования настраиваются после создания."
              : "Предыдущая публикация остаётся доступной, пока новую версию не одобрят."}
          </p>
        </div>
      </div>
    </aside>
  );
}
