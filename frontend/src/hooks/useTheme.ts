import { useContext } from 'react'
import { ThemeContext } from '../app/providers/ThemeContext'

export function useTheme() {
  const context = useContext(ThemeContext)
  if (!context) {
    throw new Error('useTheme requires ThemeProvider')
  }
  return context
}
