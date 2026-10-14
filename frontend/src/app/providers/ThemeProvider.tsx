import { useEffect, useState, type ReactNode } from "react";
import { ThemeContext, type Theme } from "./ThemeContext";

interface ThemeProviderProps {
  children: ReactNode;
}

function initialTheme(): Theme {
  try {
    const saved = localStorage.getItem("study-shelf-theme");
    if (saved === "light" || saved === "dark") {
      return saved;
    }
  } catch {
    /* Storage can be unavailable in a restricted browser. */
  }
  const prefersDarkTheme = window.matchMedia(
    "(prefers-color-scheme: dark)",
  ).matches;

  if (prefersDarkTheme) {
    return "dark";
  }

  return "light";
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  const [theme, setTheme] = useState<Theme>(initialTheme);
  useEffect(() => {
    document.documentElement.dataset.theme = theme;
    try {
      localStorage.setItem("study-shelf-theme", theme);
    } catch {
      /* Theme still works without storage. */
    }
  }, [theme]);

  const toggleTheme = () => {
    setTheme((currentTheme) => {
      if (currentTheme === "light") {
        return "dark";
      }

      return "light";
    });
  };

  return (
    <ThemeContext.Provider
      value={{
        theme,
        toggleTheme,
      }}
    >
      {children}
    </ThemeContext.Provider>
  );
}
