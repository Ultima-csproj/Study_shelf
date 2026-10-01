import { materials } from "../data/materials";
import { collectionMaterials } from "../data/collectionMaterials";
import { disciplines } from "../data/disciplines";
import { teachers } from "../data/teachers";
import type { CatalogMaterial, MaterialSource } from "../types/material";
import { getJson, HttpError } from "./http";

export async function getMaterial(
  id: string | number,
  signal?: AbortSignal,
  source: MaterialSource = "api",
): Promise<CatalogMaterial | null> {
  if (source === "mock") {
    return getDemoMaterial(id, signal);
  }

  try {
    const material = await getJson<Omit<CatalogMaterial, "source">>(
      `/api/materials/${encodeURIComponent(String(id))}`,
      signal,
    );
    return { ...material, source: "api" };
  } catch (reason) {
    if (reason instanceof HttpError && reason.status === 404) {
      return null;
    }

    throw reason;
  }
}

export async function getDemoMaterial(
  id: string | number,
  signal?: AbortSignal,
): Promise<CatalogMaterial | null> {
  await Promise.resolve();
  signal?.throwIfAborted();
  const material = [...materials, ...collectionMaterials].find(
    (item) => item.id === Number(id),
  );
  return material
    ? {
        ...material,
        source: "mock",
        discipline:
          disciplines.find((item) => item.id === material.disciplineId) ?? null,
        teacher:
          teachers.find((item) => item.id === material.teacherId) ?? null,
      }
    : null;
}
