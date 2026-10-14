import type { MaterialSummary } from "../types/material";
import { getJson } from "./http";

export interface Page<T> {
  items: T[];
  total: number;
  page: number;
  pageCount: number;
}
export interface SavedMaterial {
  id: number;
  material: MaterialSummary | null;
}
export interface SavedCollection {
  id: number;
  title: string | null;
  shortDescription: string | null;
  owned: boolean;
}
export function getSavedMaterials(page: number, signal?: AbortSignal) {
  return getJson<Page<SavedMaterial>>(
    `/api/me/library/materials?page=${page}&pageSize=20`,
    signal,
  );
}
export function getSavedCollections(page: number, signal?: AbortSignal) {
  return getJson<Page<SavedCollection>>(
    `/api/me/library/collections?page=${page}&pageSize=20`,
    signal,
  );
}
export function getContributedCollections(page: number, signal?: AbortSignal) {
  return getJson<Page<SavedCollection>>(
    `/api/me/library/contributions?page=${page}&pageSize=20`,
    signal,
  );
}
