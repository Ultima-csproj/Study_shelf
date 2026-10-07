import { getJson, mutateJson } from "./http";
import type { Page } from "./library";
import type { MaterialSummary } from "../types/material";

export interface CollectionInfo {
  title: string;
  shortDescription: string;
  description: string;
  purpose: string;
  targetAudience: string;
}
export interface PersonalCollection extends CollectionInfo {
  id: number;
  visibility: "private" | "public";
  editVersion: number;
}
export interface PersonalSection {
  id: number;
  title: string;
  description: string;
  position: number;
  entryCount: number;
}
export interface PersonalDetail {
  collection: PersonalCollection;
  sections: PersonalSection[];
}
export interface PersonalEntry {
  id: number;
  materialId: number;
  position: number;
  material: MaterialSummary | null;
}
export type CollectionAction =
  | ({ action: "info" } & CollectionInfo)
  | { action: "add_section"; title: string; description: string }
  | { action: "update_section"; id: number; title: string; description: string }
  | { action: "delete_section"; id: number }
  | { action: "add_entry"; section: number; material: number }
  | { action: "delete_entry"; section: number; id: number }
  | { action: "reorder_sections"; ids: number[] }
  | { action: "reorder_entries"; section: number; ids: number[] };

export function getPersonalCollections(page: number, signal?: AbortSignal) {
  return getJson<Page<PersonalCollection>>(
    `/api/me/collections?page=${page}&pageSize=20`,
    signal,
  );
}
export function createCollection(info: CollectionInfo) {
  return mutateJson<PersonalDetail>("/api/me/collections", "POST", info);
}
export function getPersonalCollection(id: number, signal?: AbortSignal) {
  return getJson<PersonalDetail>(`/api/me/collections/${id}`, signal);
}
export function editCollection(
  id: number,
  expectedVersion: number,
  action: CollectionAction,
) {
  return mutateJson<PersonalDetail>(`/api/me/collections/${id}`, "PATCH", {
    expectedVersion,
    change: action,
  });
}
export function getPersonalEntries(
  id: number,
  section: number,
  page: number,
  signal?: AbortSignal,
) {
  return getJson<Page<PersonalEntry>>(
    `/api/me/collections/${id}/sections/${section}/items?page=${page}&pageSize=100`,
    signal,
  );
}
