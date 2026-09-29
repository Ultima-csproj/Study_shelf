import { RouterProvider } from 'react-router-dom'
import { ThemeProvider } from './providers/ThemeProvider'
import { router } from './router'
import { ViewerProvider } from './providers/ViewerProvider'

export function App() {
  return (
    <ThemeProvider>
      <ViewerProvider>
        <RouterProvider router={router} />
      </ViewerProvider>
    </ThemeProvider>
  )
}
