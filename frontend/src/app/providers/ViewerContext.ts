import { createContext } from "react";
import type { User, Viewer } from "../../types/viewer";
import type { MaterialReference } from "../../types/material";
export interface ViewerContextValue {
  viewer: Viewer;
  savedMaterials: MaterialReference[];
  savedCollectionIds: number[];
  signInDemo: () => void;
  signOut: () => void;
  toggleSaved: (material: MaterialReference) => void;
  toggleSavedCollection: (id: number) => void;
  updateUser: (user: User) => void;
}
export const ViewerContext = createContext<ViewerContextValue | null>(null);
