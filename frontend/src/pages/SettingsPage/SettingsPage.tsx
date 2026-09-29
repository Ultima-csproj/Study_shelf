import { ThemeToggle } from '../../components/common/ThemeToggle/ThemeToggle'
import styles from './SettingsPage.module.css'

export function SettingsPage() {
  return (
    <div className={styles.page}>
      <h1>Настройки</h1>
      <section>
        <h2>Внешний вид</h2>
        <div className={styles.appearance}>
          <p>Светлая или тёмная тема</p>
          <ThemeToggle />
        </div>
      </section>
      <section>
        <h2>Аккаунт</h2>
        <p>
          Email, пароль и безопасность появятся после подключения настоящего
          аккаунта.
        </p>
      </section>
    </div>
  )
}
