import {
  useCallback,
  useEffect,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { ViewerContext } from "./ViewerContext";
import {
  enter,
  getViewer,
  leave,
  type LoginInput,
  type RegisterInput,
} from "../../api/auth";
import { HttpError, mutateJson } from "../../api/http";
import type { User, Viewer } from "../../types/viewer";
import type { MaterialReference } from "../../types/material";

interface ViewerProviderProps {
  children: ReactNode;
}

export function ViewerProvider({ children }: ViewerProviderProps) {
  const [viewer, setViewer] = useState<Viewer>({ kind: "loading" });
  const [savedFlags, setSavedFlags] = useState<Record<string, boolean>>({});
  const [libraryVersion, setLibraryVersion] = useState(0);
  const [actionError, setActionError] = useState<string | null>(null);
  const generation = useRef(0);
  const flags = useRef<Record<string, boolean>>({});
  const queued = useRef(new Set<string>());
  const requested = useRef(new Set<string>());
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const authenticated = useRef(false);
  const currentUser = useRef<number | null>(null);

  const reset = useCallback(() => {
    generation.current += 1;
    flags.current = {};
    queued.current.clear();
    requested.current.clear();
    setSavedFlags({});
    setLibraryVersion((value) => value + 1);
    setActionError(null);
  }, []);

  const refreshViewer = useCallback(async () => {
    const version = ++generation.current;
    try {
      const result = await getViewer();
      if (version === generation.current) {
        const user = result.kind === "authenticated" ? result.user.id : null;
        if (currentUser.current !== user) {
          reset();
          currentUser.current = user;
        }
        authenticated.current = result.kind === "authenticated";
        setViewer(result);
      }
    } catch (reason) {
      if (version === generation.current) {
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
      generation.current += 1;
      if (timer.current !== null) {
        clearTimeout(timer.current);
        timer.current = null;
      }
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

  const registerSave = useCallback(
    (kind: "material" | "collection", id: number) => {
      const key = `${kind}:${id}`;
      if (!authenticated.current || requested.current.has(key)) {
        return;
      }
      requested.current.add(key);
      queued.current.add(key);
      if (timer.current !== null) {
        return;
      }
      timer.current = setTimeout(async () => {
        timer.current = null;
        const keys = Array.from(queued.current);
        queued.current.clear();
        const version = generation.current;
        for (let offset = 0; offset < keys.length; offset += 100) {
          const chunk = keys.slice(offset, offset + 100);
          const materialIds = chunk
            .filter((k) => k.startsWith("material:"))
            .map((k) => Number(k.split(":")[1]));
          const collectionIds = chunk
            .filter((k) => k.startsWith("collection:"))
            .map((k) => Number(k.split(":")[1]));
          try {
            const result = await mutateJson<{
              materialIds: number[];
              collectionIds: number[];
            }>("/api/me/library/flags", "POST", { materialIds, collectionIds });
            if (version !== generation.current) {
              return;
            }
            const present = new Set([
              ...result.materialIds.map((id) => `material:${id}`),
              ...result.collectionIds.map((id) => `collection:${id}`),
            ]);
            const next = { ...flags.current };
            for (const key of chunk) {
              next[key] = present.has(key);
            }
            flags.current = next;
            setSavedFlags(next);
          } catch (reason) {
            if (version === generation.current) {
              for (const key of chunk) {
                requested.current.delete(key);
              }
              setActionError(
                reason instanceof Error
                  ? reason.message
                  : "Не удалось загрузить сохранения",
              );
            }
          }
        }
      }, 0);
    },
    [],
  );

  const toggle = async (kind: "material" | "collection", id: number) => {
    const key = `${kind}:${id}`;
    const current = flags.current[key];
    if (current === undefined || !authenticated.current) {
      return;
    }
    const version = generation.current;
    try {
      await mutateJson(
        `/api/me/library/${kind === "material" ? "materials" : "collections"}/${id}`,
        current ? "DELETE" : "PUT",
      );
      if (version === generation.current) {
        flags.current = { ...flags.current, [key]: !current };
        setSavedFlags(flags.current);
        setLibraryVersion((value) => value + 1);
      }
    } catch (reason) {
      if (
        version === generation.current &&
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
  const value = {
    viewer,
    libraryVersion,
    signIn,
    signOut,
    refreshViewer,
    registerSave,
    savedFlags,
    toggleSaved: (material: MaterialReference) =>
      toggle("material", material.id),
    toggleSavedCollection: (id: number) => toggle("collection", id),
    updateUser,
  };

  let errorNotice: ReactNode = null;
  const retryAccount = () => {
    reset();
    void refreshViewer();
  };
  if (actionError) {
    errorNotice = (
      <p role="alert">
        {actionError}
        <button type="button" onClick={retryAccount}>
          Повторить
        </button>
      </p>
    );
  }

  return (
    <ViewerContext.Provider value={value}>
      {errorNotice}
      {children}
    </ViewerContext.Provider>
  );
}
