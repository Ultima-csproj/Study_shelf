import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { App } from './app/App'
import '@fontsource/arimo/latin-400.css'
import '@fontsource/arimo/cyrillic-400.css'
import '@fontsource/arimo/latin-500.css'
import '@fontsource/arimo/cyrillic-500.css'
import '@fontsource/arimo/latin-600.css'
import '@fontsource/arimo/cyrillic-600.css'
import '@fontsource/arimo/latin-700.css'
import '@fontsource/arimo/cyrillic-700.css'
import './styles/globals.css'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
