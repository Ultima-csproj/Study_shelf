import { SunIcon } from '@phosphor-icons/react/dist/csr/Sun'
import { MoonIcon } from '@phosphor-icons/react/dist/csr/Moon'
import { useTheme } from '../../../hooks/useTheme'
import styles from './ThemeToggle.module.css'

export function ThemeToggle() {
  const { theme, toggleTheme } = useTheme()
  const light = theme === 'light'
  const title = light ? 'Включить тёмную тему' : 'Включить светлую тему'
  const trackClassName = `${styles.track} ${light ? styles.light : ''}`
  return (
    <button
      type="button"
      role="switch"
      aria-label="Светлая тема"
      aria-checked={light}
      onClick={toggleTheme}
      title={title}
      className={styles.control}
    >
      {light ? (
        <SunIcon
          size={20}
          aria-hidden="true"
        />
      ) : (
        <MoonIcon
          size={20}
          aria-hidden="true"
        />
      )}
      <span
        className={trackClassName}
        aria-hidden="true"
      >
        <span className={styles.thumb} />
      </span>
    </button>
  )
}
