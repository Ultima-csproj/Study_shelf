import {
  useCallback,
  useEffect,
  useRef,
  useState,
  type RefObject,
} from "react";
import { mutateJson } from "../../api/http";

export type SavedKind = "material" | "collection";
export type SavedState =
  | { status: "loading" }
  | { status: "ready"; saved: boolean; busy: boolean }
  | { status: "error"; message: string };

interface FlagsResponse {
  materialIds: number[];
  collectionIds: number[];
}

const CACHE_TTL = 60_000;

export function useSavedLibrary(authenticated: RefObject<boolean>) {
  const [savedStates, setSavedStates] = useState<Record<string, SavedState>>(
    {},
  );
  const [libraryVersion, setLibraryVersion] = useState(0);
  const states = useRef<Record<string, SavedState>>({});
  const loadedAt = useRef(new Map<string, number>());
  const serials = useRef(new Map<string, number>());
  const queue = useRef(new Set<string>());
  const generation = useRef(0);
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);

  const publish = useCallback(() => {
    setSavedStates({ ...states.current });
  }, []);

  const resetLibrary = useCallback(() => {
    generation.current += 1;
    if (timer.current !== null) {
      clearTimeout(timer.current);
      timer.current = null;
    }
    queue.current.clear();
    loadedAt.current.clear();
    serials.current.clear();
    states.current = {};
    setSavedStates({});
    setLibraryVersion((value) => value + 1);
  }, []);

  const registerSave = useCallback(
    (kind: SavedKind, id: number) => {
      const key = `${kind}:${id}`;
      const state = states.current[key];
      if (
        !authenticated.current ||
        state?.status === "loading" ||
        state?.status === "error"
      ) {
        return;
      }
      if (
        state?.status === "ready" &&
        (state.busy ||
          Date.now() - (loadedAt.current.get(key) ?? 0) < CACHE_TTL)
      ) {
        return;
      }
      states.current[key] = { status: "loading" };
      queue.current.add(key);
      publish();
      if (timer.current !== null) {
        return;
      }
      timer.current = setTimeout(async () => {
        timer.current = null;
        const keys = Array.from(queue.current);
        queue.current.clear();
        const epoch = generation.current;
        for (let offset = 0; offset < keys.length; offset += 100) {
          if (epoch !== generation.current || !authenticated.current) {
            return;
          }
          const chunk = keys.slice(offset, offset + 100);
          const versions = new Map(
            chunk.map((key) => [key, serials.current.get(key) ?? 0]),
          );
          const materialIds = chunk
            .filter((key) => key.startsWith("material:"))
            .map((key) => Number(key.split(":")[1]));
          const collectionIds = chunk
            .filter((key) => key.startsWith("collection:"))
            .map((key) => Number(key.split(":")[1]));
          try {
            const result = await mutateJson<FlagsResponse>(
              "/api/me/library/flags",
              "POST",
              {
                materialIds,
                collectionIds,
              },
            );
            if (epoch !== generation.current) {
              return;
            }
            const present = new Set([
              ...result.materialIds.map((id) => `material:${id}`),
              ...result.collectionIds.map((id) => `collection:${id}`),
            ]);
            for (const key of chunk) {
              if (versions.get(key) !== (serials.current.get(key) ?? 0)) {
                continue;
              }
              states.current[key] = {
                status: "ready",
                saved: present.has(key),
                busy: false,
              };
              loadedAt.current.set(key, Date.now());
            }
            publish();
          } catch (reason) {
            if (epoch !== generation.current) {
              return;
            }
            const message =
              reason instanceof Error
                ? reason.message
                : "Не удалось проверить сохранение";
            for (const key of chunk) {
              if (versions.get(key) === (serials.current.get(key) ?? 0)) {
                states.current[key] = { status: "error", message };
              }
            }
            publish();
          }
        }
      }, 0);
    },
    [authenticated, publish],
  );

  const retrySave = useCallback(
    (kind: SavedKind, id: number) => {
      const key = `${kind}:${id}`;
      if (states.current[key]?.status !== "error") {
        return;
      }
      delete states.current[key];
      registerSave(kind, id);
    },
    [registerSave],
  );

  const toggle = async (kind: SavedKind, id: number) => {
    const key = `${kind}:${id}`;
    const state = states.current[key];
    if (!authenticated.current || state?.status !== "ready" || state.busy) {
      return;
    }
    const epoch = generation.current;
    serials.current.set(key, (serials.current.get(key) ?? 0) + 1);
    states.current[key] = { ...state, busy: true };
    publish();
    try {
      await mutateJson(
        `/api/me/library/${kind === "material" ? "materials" : "collections"}/${id}`,
        state.saved ? "DELETE" : "PUT",
      );
      if (epoch === generation.current) {
        states.current[key] = {
          status: "ready",
          saved: !state.saved,
          busy: false,
        };
        loadedAt.current.set(key, Date.now());
        publish();
        setLibraryVersion((value) => value + 1);
      }
    } catch (reason) {
      if (epoch === generation.current) {
        states.current[key] = state;
        publish();
      }
      throw reason;
    }
  };

  useEffect(() => {
    const pendingQueue = queue.current;
    const refresh = () => {
      if (document.visibilityState !== "visible" || !authenticated.current) {
        return;
      }
      loadedAt.current.clear();
      setLibraryVersion((value) => value + 1);
    };
    window.addEventListener("focus", refresh);
    document.addEventListener("visibilitychange", refresh);
    return () => {
      window.removeEventListener("focus", refresh);
      document.removeEventListener("visibilitychange", refresh);
      generation.current += 1;
      if (timer.current !== null) {
        clearTimeout(timer.current);
        timer.current = null;
      }
      // StrictMode repeats the mount effects; abandoned reads must be eligible again.
      pendingQueue.clear();
      states.current = {};
    };
  }, [authenticated]);

  return {
    savedStates,
    libraryVersion,
    registerSave,
    retrySave,
    resetLibrary,
    toggle,
  };
}
