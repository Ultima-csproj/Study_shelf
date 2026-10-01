import { getCollections } from "./collections";
import { getMaterial } from "./materials";
import type { CollectionSummary } from "../types/collection";
import type { CatalogMaterial, MaterialReference } from "../types/material";

export interface LibraryResponse {
  materials: CatalogMaterial[];
  savedCollections: CollectionSummary[];
  contributedCollections: CollectionSummary[];
}

export async function getLibrary(
  userId: number,
  savedMaterials: MaterialReference[],
  collectionIds: number[],
  signal?: AbortSignal,
): Promise<LibraryResponse> {
  const [materialResults, collectionResults] = await Promise.all([
    Promise.all(
      savedMaterials.map((material) => {
        return getMaterial(material.id, signal, material.source);
      }),
    ),
    getCollections(
      { query: "", disciplineIds: [], courses: [], sort: "newest" },
      signal,
    ),
  ]);

  signal?.throwIfAborted();

  return {
    materials: materialResults.filter(
      (material): material is CatalogMaterial => material !== null,
    ),
    savedCollections: collectionResults.items.filter((collection) =>
      collectionIds.includes(collection.id),
    ),
    contributedCollections: collectionResults.items.filter((collection) => {
      return collection.contributors.some((contributor) => {
        const identity = contributor.identity;
        return identity.kind === "user" && identity.userId === userId;
      });
    }),
  };
}
