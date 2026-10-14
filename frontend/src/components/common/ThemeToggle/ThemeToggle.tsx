import { SunIcon } from "@phosphor-icons/react/dist/csr/Sun";
import { MoonIcon } from "@phosphor-icons/react/dist/csr/Moon";
import type { ReactNode } from "react";
import { useTheme } from "../../../hooks/useTheme";
import styles from "./ThemeToggle.module.css";

export function ThemeToggle() {
  const { theme, toggleTheme } = useTheme();
  const light = theme === "light";
  let title = "Включить светлую тему";

  if (light) {
    title = "Включить тёмную тему";
  }

  const trackClassNames = [styles.track];

  if (!light) {
    trackClassNames.push(styles.light);
  }

  const trackClassName = trackClassNames.join(" ");
  let themeIcon: ReactNode;

  if (light) {
    themeIcon = <SunIcon size={20} aria-hidden="true" />;
  } else {
    themeIcon = <MoonIcon size={20} aria-hidden="true" />;
  }

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
      {themeIcon}
      <span className={trackClassName} aria-hidden="true">
        <span className={styles.thumb} />
      </span>
    </button>
  );
}
