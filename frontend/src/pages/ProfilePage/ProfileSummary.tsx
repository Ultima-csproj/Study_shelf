import { LockSimpleIcon } from "@phosphor-icons/react/dist/csr/LockSimple";
import type { User } from "../../types/viewer";
import styles from "./ProfilePage.module.css";

interface ProfileSummaryProps {
  user: User;
}
export function ProfileSummary({ user }: ProfileSummaryProps) {
  return (
    <>
      <section className={styles.section}>
        <h2>Личная информация</h2>
        <dl className={styles.personalFields}>
          <div>
            <dt>Имя</dt>
            <dd>{user.firstName}</dd>
          </div>
          <div>
            <dt>Фамилия</dt>
            <dd>{user.lastName || "Не добавлена"}</dd>
          </div>
        </dl>
        <p className={styles.privacy}>
          <LockSimpleIcon size={16} aria-hidden="true" />
          Эти данные видны только вам и сотрудникам с разрешением на их
          просмотр.
        </p>
      </section>
      {user.role === "student" && (
        <section className={styles.section}>
          <h2>Обучение</h2>
          <dl className={styles.personalFields}>
            <div>
              <dt>Курс</dt>
              <dd>
                {user.course === null ? "Не добавлен" : `${user.course} курс`}
              </dd>
            </div>
            <div>
              <dt>Группа</dt>
              <dd>{user.group || "Не добавлена"}</dd>
            </div>
          </dl>
        </section>
      )}
    </>
  );
}
