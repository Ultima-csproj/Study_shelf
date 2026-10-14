import {
  useCallback,
  useEffect,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { ViewerContext } from "./ViewerContext";
import { useSavedLibrary, type SavedKind } from "./useSavedLibrary";
import {
  enter,
  getViewer,
  leave,
  type LoginInput,
  type RegisterInput,
} from "../../api/auth";
import { HttpError } from "../../api/http";
import type { User, Viewer } from "../../types/viewer";
import type { MaterialReference } from "../../types/material";

export function ViewerProvider({ children }: { children: ReactNode }) {
  const [viewer, setViewer] = useState<Viewer>({ kind: "loading" });
  const [actionError, setActionError] = useState<string | null>(null);
  const viewerRequest = useRef(0);
  const authenticated = useRef(false);
  const currentUser = useRef<number | null>(null);
  const {
    savedStates,
    libraryVersion,
    registerSave,
    retrySave,
    resetLibrary,
    toggle,
  } = useSavedLibrary(authenticated);

  const reset = useCallback(() => {
    viewerRequest.current += 1;
    resetLibrary();
    setActionError(null);
  }, [resetLibrary]);

  const refreshViewer = useCallback(async () => {
    const version = ++viewerRequest.current;
    try {
      const result = await getViewer();
      if (version !== viewerRequest.current) {
        return;
      }
      const user = result.kind === "authenticated" ? result.user.id : null;
      if (currentUser.current !== user) {
        reset();
        currentUser.current = user;
      }
      authenticated.current = result.kind === "authenticated";
      setViewer(result);
    } catch (reason) {
      if (version === viewerRequest.current) {
        authenticated.current = false;
        setViewer({
          kind: "error",
          message:
            reason instanceof Error
              ? reason.message
              : "Не удалось загрузить аккаунт",
        });
      }
    }
  }, [reset]);

  useEffect(() => {
    let mounted = true;
    queueMicrotask(() => {
      if (mounted) {
        void refreshViewer();
      }
    });
    return () => {
      mounted = false;
      viewerRequest.current += 1;
    };
  }, [refreshViewer]);

  const signIn = async (
    input: LoginInput | RegisterInput,
    register: boolean,
  ) => {
    const result = await enter(input, register);
    reset();
    authenticated.current = result.kind === "authenticated";
    currentUser.current =
      result.kind === "authenticated" ? result.user.id : null;
    setViewer(result);
  };

  const signOut = async () => {
    try {
      await leave();
      reset();
      authenticated.current = false;
      currentUser.current = null;
      setViewer({ kind: "guest" });
    } catch (reason) {
      setActionError(
        reason instanceof Error ? reason.message : "Не удалось выйти",
      );
    }
  };

  const toggleSaved = async (kind: SavedKind, id: number) => {
    const user = currentUser.current;
    try {
      await toggle(kind, id);
    } catch (reason) {
      if (
        user === currentUser.current &&
        reason instanceof HttpError &&
        reason.status === 401
      ) {
        reset();
        authenticated.current = false;
        currentUser.current = null;
        setViewer({ kind: "guest" });
      }
      throw reason;
    }
  };

  const updateUser = (user: User) => {
    setViewer((current) => {
      if (current.kind !== "authenticated" || current.user.id !== user.id) {
        return current;
      }
      return { kind: "authenticated", user };
    });
  };
  const retryAccount = () => {
    reset();
    void refreshViewer();
  };
  const value = {
    viewer,
    libraryVersion,
    signIn,
    signOut,
    refreshViewer,
    registerSave,
    retrySave,
    savedStates,
    toggleSaved: (material: MaterialReference) =>
      toggleSaved("material", material.id),
    toggleSavedCollection: (id: number) => toggleSaved("collection", id),
    updateUser,
  };

  return (
    <ViewerContext.Provider value={value}>
      {actionError && (
        <p role="alert">
          {actionError}
          <button type="button" onClick={retryAccount}>
            Повторить
          </button>
        </p>
      )}
      {children}
    </ViewerContext.Provider>
  );
}
