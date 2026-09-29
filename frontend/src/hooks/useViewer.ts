import { useContext } from 'react'
import { ViewerContext } from '../app/providers/ViewerContext'
export function useViewer() {
  const context = useContext(ViewerContext)
  if (!context) {
    throw new Error('useViewer requires ViewerProvider')
  }
  return context
}
