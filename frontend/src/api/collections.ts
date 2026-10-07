import { getJson, HttpError } from "./http";
import { collectionItem, materialMap } from "./learning";
import type { CollectionDetail, CollectionRequest, CollectionResponse, CollectionSummary } from "../types/collection";
import type { LearningDetail, SectionPage } from "../types/learning";

export function getCollections(request: CollectionRequest, signal?: AbortSignal): Promise<CollectionResponse> {
  const params = new URLSearchParams({
    sort: request.sort,
    page: String(request.page),
    pageSize: String(request.pageSize),
  });
  if (request.query.trim()) {
    params.set("q", request.query.trim());
  }
  for (const id of request.disciplineIds) {
    params.append("discipline", String(id));
  }
  for (const course of request.courses) {
    params.append("course", String(course));
  }
  return getJson("/api/collections?" + params, signal);
}
export async function getCollection(id: string | number, signal?: AbortSignal): Promise<CollectionDetail | null> {
  try {
    const response = await getJson<CollectionSummary & LearningDetail>(
      "/api/collections/" + encodeURIComponent(String(id)),
      signal,
    );
    const materials = materialMap(response.materials);
    return {
      ...response,
      sections: response.sections.map((section) => {
        return {
          ...section,
          initialPageSize: response.initialPageSize,
          items: section.items.map((entry) => collectionItem(entry, materials)),
        };
      }),
    };
  } catch (reason) {
    if (reason instanceof HttpError && reason.status === 404) {
      return null;
    }
    throw reason;
  }
}
export async function getCollectionSectionItems(
  id: number,
  section: number,
  page: number,
  pageSize: number,
  signal?: AbortSignal,
) {
  const response = await getJson<SectionPage>(
    `/api/collections/${id}/sections/${section}/items?page=${page}&pageSize=${pageSize}`,
    signal,
  );
  const materials = materialMap(response.materials);
  return {
    ...response,
    items: response.items.map((entry) => collectionItem(entry, materials)),
  };
}
