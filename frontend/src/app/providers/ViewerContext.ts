import { createContext } from 'react'
import type { User, Viewer } from '../../types/viewer'
export interface ViewerContextValue {
  viewer: Viewer
  savedMaterialIds: number[]
  savedCollectionIds: number[]
  signInDemo: () => void
  signOut: () => void
  toggleSaved: (id: number) => void
  toggleSavedCollection: (id: number) => void
  updateUser: (user: User) => void
}
export const ViewerContext = createContext<ViewerContextValue | null>(null)
