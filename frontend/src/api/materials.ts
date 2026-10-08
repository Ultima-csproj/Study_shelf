import type {
  CatalogMaterial,
  MaterialSource,
  MaterialDetail,
} from "../types/material";
import { getJson, HttpError } from "./http";

export async function getMaterial(
  id: string | number,
  signal?: AbortSignal,
  source: MaterialSource = "api",
  allowPrivate = false,
  context = "",
): Promise<CatalogMaterial | null> {
  // Retired demo references never resolve to an unrelated real API material.
  if (source === "mock") {
    return null;
  }
  try {
    const material = await getJson<MaterialDetail>(
      "/api/materials/" + encodeURIComponent(String(id)) + "?" + context.replace(/^&/, ""),
      signal,
    );
    return { ...material, source: "api" };
  } catch (reason) {
    if (reason instanceof HttpError && reason.status === 404) {
      if (!allowPrivate) {
        return null;
      }
      try {
        const material = await getJson<MaterialDetail>(
          "/api/me/materials/" + encodeURIComponent(String(id)) + "?" + context.replace(/^&/, ""),
          signal,
        );
        return { ...material, source: "api" };
      } catch (privateError) {
        if (
          privateError instanceof HttpError &&
          [401, 404].includes(privateError.status)
        ) {
          return null;
        }
        throw privateError;
      }
    }
    throw reason;
  }
}
