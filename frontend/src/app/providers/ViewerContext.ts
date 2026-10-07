import { createContext } from "react";
import type { User, Viewer } from "../../types/viewer";
import type { MaterialReference } from "../../types/material";
import type { LoginInput, RegisterInput } from "../../api/auth";

export interface ViewerContextValue {
  viewer: Viewer;
  libraryVersion: number;
  signIn: (
    input: LoginInput | RegisterInput,
    register: boolean,
  ) => Promise<void>;
  signOut: () => Promise<void>;
  refreshViewer: () => Promise<void>;
  registerSave: (kind: "material" | "collection", id: number) => void;
  savedFlags: Record<string, boolean>;
  toggleSaved: (material: MaterialReference) => Promise<void>;
  toggleSavedCollection: (id: number) => Promise<void>;
  updateUser: (user: User) => void;
}
export const ViewerContext = createContext<ViewerContextValue | null>(null);
