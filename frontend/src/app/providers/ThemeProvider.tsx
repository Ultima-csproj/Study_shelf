import { useEffect, useState, type ReactNode } from 'react'
import { ThemeContext, type Theme } from './ThemeContext'

function initialTheme(): Theme {
  try {
    const saved = localStorage.getItem('study-shelf-theme')
    if (saved === 'light' || saved === 'dark') {
      return saved
    }
  } catch {
    /* Storage can be unavailable in a restricted browser. */
  }
  return window.matchMedia('(prefers-color-scheme: dark)').matches
    ? 'dark'
    : 'light'
}

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [theme, setTheme] = useState<Theme>(initialTheme)
  useEffect(() => {
    document.documentElement.dataset.theme = theme
    try {
      localStorage.setItem('study-shelf-theme', theme)
    } catch {
      /* Theme still works without storage. */
    }
  }, [theme])
  return (
    <ThemeContext.Provider
      value={{
        theme,
        toggleTheme: () =>
          setTheme((current) => (current === 'light' ? 'dark' : 'light')),
      }}
    >
      {children}
    </ThemeContext.Provider>
  )
}
