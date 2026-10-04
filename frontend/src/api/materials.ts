import { materials } from "../data/materials";
import { collectionMaterials } from "../data/collectionMaterials";
import { disciplines } from "../data/disciplines";
import { teachers } from "../data/teachers";
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
): Promise<CatalogMaterial | null> {
  if (source === "mock") {
    return getDemoMaterial(id, signal);
  }
  try {
    const material = await getJson<MaterialDetail>(
      "/api/materials/" + encodeURIComponent(String(id)),
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

// Explicit presentation adapter for isolated demonstration pages, never an API fallback.
export async function getDemoMaterial(
  id: string | number,
  signal?: AbortSignal,
): Promise<CatalogMaterial | null> {
  await Promise.resolve();
  signal?.throwIfAborted();
  const numericId = Number(id);
  const material =
    materials.find((item) => item.id === numericId) ??
    collectionMaterials.find((item) => item.id === numericId);
  if (!material) {
    return null;
  }
  const discipline = disciplines.find((item) => item.id === material.disciplineId);
  const teacher = teachers.find((item) => item.id === material.teacherId);
  let displayName = material.authorName ?? "Автор не указан";
  if (!material.authorName && teacher) {
    displayName = teacher.name;
  }
  let durationSeconds: number | null = null;
  if (material.duration) {
    durationSeconds = material.duration.split(":").reduce((total, part) => {
      return total * 60 + Number(part);
    }, 0);
  }
  const publishedAt = material.createdAt + "T00:00:00Z";
  let firstPublishedAt = publishedAt;
  if (material.dateLabel === "Обновлено") {
    firstPublishedAt = new Date(
      new Date(publishedAt).getTime() - 86400000,
    ).toISOString();
  }
  return {
    id: material.id,
    publishedRevisionId: null,
    title: material.title,
    description: material.description,
    type: material.type,
    tags: material.tags,
    credits: [{ contributorId: null, displayName, role: "author" }],
    placements: discipline ? [{
      discipline: { id: discipline.id, slug: discipline.slug, name: discipline.name },
      responsible: teacher ?? null,
      course: material.course,
    }] : [],
    sourceYear: material.year,
    publishedAt,
    firstPublishedAt,
    preview: material.preview ?? null,
    durationSeconds,
    fileSizeBytes: null,
    source: "mock",
  };
}
