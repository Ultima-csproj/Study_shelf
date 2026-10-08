import { useEffect, useState } from "react";
import { getMaterial } from "../api/materials";
import type { CatalogMaterial, MaterialSource } from "../types/material";
import { useViewer } from "./useViewer";
import { useLocation } from "react-router-dom";
import { materialContext } from "../utils/materialContext";

export function useMaterial(id: string, source: MaterialSource) {
  const { viewer } = useViewer();
  const location = useLocation();
  const context = materialContext(location.search);
  const account =
    viewer.kind === "authenticated" ? String(viewer.user.id) : viewer.kind;
  const allowPrivate = viewer.kind === "authenticated";
  const [attempt, setAttempt] = useState(0);
  const key = `${account}:${source}:${id}:${context}:${attempt}`;
  const [state, setState] = useState<{
    key: string;
    material: CatalogMaterial | null;
    error: string | null;
  }>({ key: "", material: null, error: null });
  useEffect(() => {
    const controller = new AbortController();
    getMaterial(id, controller.signal, source, allowPrivate, context)
      .then((material) => {
        if (!controller.signal.aborted) {
          setState({ key, material, error: null });
        }
      })
      .catch((reason: unknown) => {
        if (!controller.signal.aborted) {
          setState({
            key,
            material: null,
            error:
              reason instanceof Error
                ? reason.message
                : "Не удалось загрузить материал",
          });
        }
      });
    return () => controller.abort();
  }, [id, source, key, allowPrivate, context]);
  return {
    material: state.key === key ? state.material : null,
    error: state.key === key ? state.error : null,
    loading: state.key !== key,
    retry: () => setAttempt((value) => value + 1),
  };
}
