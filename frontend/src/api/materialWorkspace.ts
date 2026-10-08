import { getJson, mutateJson } from "./http";
import type { Page } from "./library";
import type { AdminQuery } from "./admin";
import type { ContributorRole, MaterialType } from "../types/material";

export interface MaterialHeader {
  id: number;
  title: string;
  lifecycle: "active" | "archived";
  publication: "published" | "unpublished";
  visibility: "public" | "private";
  version: number;
  ownerUserId: number | null;
  ownerInstitutionId: number | null;
  ownerName: string;
  publishedRevisionId: number | null;
  workingRevisionId: number | null;
  workingRevisionState: string | null;
}
export interface RevisionHeader {
  id: number;
  title: string;
  state: string;
  version: number;
  baseRevisionId: number | null;
  createdAt: string;
  publishedAt: string | null;
}
export interface CreditInput {
  contributorId: number;
  role: ContributorRole;
}
export interface ContentInput {
  title: string;
  description: string;
  materialType: MaterialType;
  sourceYear: number | null;
  previewUrl: string | null;
  durationSeconds: number | null;
  fileSizeBytes: number | null;
  tags: string[];
  credits: CreditInput[];
}
export interface Revision extends ContentInput {
  id: number;
  materialId: number;
  version: number;
  state: string;
  baseRevisionId: number | null;
  createdAt: string;
  updatedAt: string;
  submittedAt: string | null;
  publishedAt: string | null;
  credits: (CreditInput & { displayName: string })[];
}
export interface Restriction {
  id: number;
  institutionId: number | null;
  institutionName: string | null;
  reason: string;
  createdAt: string;
}
export interface RestrictionTarget {
  material: MaterialHeader;
  global: boolean;
  institutions: number[];
}
export interface Placement {
  id: number;
  disciplineId: number;
  disciplineName: string;
  institutionId: number;
  institutionName: string;
  entryCount: number;
  canDetach: boolean;
}
export interface PlacementEntry {
  id: number;
  sectionId: number;
  sectionTitle: string;
  position: number;
}
export interface Workspace {
  material: MaterialHeader;
  revisions: Page<RevisionHeader>;
  restrictions: Restriction[];
  placements: Placement[];
  capabilities: {
    manage: boolean;
    edit: boolean;
    submit: boolean;
    collaborators: boolean;
    restrictGlobal: boolean;
    restrictInstitutions: number[];
  };
}
export interface RevisionResponse {
  material: MaterialHeader;
  revision: Revision;
  outdated: boolean;
  selfReview: boolean;
  canModerate: boolean;
  decision: {
    reviewerId: number;
    decision: string;
    reason: string | null;
    reviewedAt: string;
    wasSelfReview: boolean;
  } | null;
}
export interface Collaborator {
  userId: number;
  name: string;
  active: boolean;
  permissions: string[];
}
export interface MaterialEvent {
  id: number;
  revisionId: number | null;
  actorName: string;
  action: string;
  reason: string;
  createdAt: string;
}
export interface PublishedHistory {
  materialId: number;
  lifecycle: string;
  currentRevisionId: number | null;
  revisions: Page<RevisionHeader>;
}
export const revisionLabels: Record<string, string> = {
  draft: "Черновик",
  pending_moderation: "На модерации",
  published: "Опубликована",
  rejected: "Отклонена",
  discarded: "Черновик отменён",
};
export const collaboratorLabels: Record<string, string> = {
  read: "Читать материал и редакции",
  edit_revision: "Редактировать черновик",
  submit_for_moderation: "Отправлять на модерацию",
  manage_collaborators: "Управлять приглашёнными",
};
export const creditLabels: Record<ContributorRole, string> = {
  author: "Автор",
  co_author: "Соавтор",
  editor: "Редактор",
  translator: "Переводчик",
  lecturer: "Лектор",
  reviewer: "Рецензент",
};
function params(query: AdminQuery) {
  const result = new URLSearchParams({
    page: String(query.page),
    pageSize: "20",
  });
  if (query.q) {
    result.set("q", query.q);
  }
  if (query.state) {
    result.set("state", query.state);
  }
  return result;
}
const base = (id: number) => "/api/me/materials/" + id;
export function getAuthorMaterials(query: AdminQuery, signal?: AbortSignal) {
  return getJson<Page<MaterialHeader>>(
    "/api/me/author-materials?" + params(query),
    signal,
  );
}
export function getModerationMaterials(
  query: AdminQuery,
  signal?: AbortSignal,
) {
  return getJson<Page<MaterialHeader>>(
    "/api/admin/materials/pending?" + params(query),
    signal,
  );
}
export function getRestrictedMaterials(
  query: AdminQuery,
  signal?: AbortSignal,
) {
  return getJson<Page<MaterialHeader>>(
    "/api/admin/materials/restricted?" + params(query),
    signal,
  );
}
export function getRestrictionMetadata(id: number, signal?: AbortSignal) {
  return getJson<Restriction[]>(base(id) + "/restrictions", signal);
}
export function getRestrictionTarget(id: number, signal?: AbortSignal) {
  return getJson<RestrictionTarget>(
    "/api/admin/materials/" + id + "/restriction-target",
    signal,
  );
}
export function getWorkspace(id: number, page: number, signal?: AbortSignal) {
  return getJson<Workspace>(base(id) + "/workspace?page=" + page, signal);
}
export function getRevision(
  id: number,
  revision: number,
  signal?: AbortSignal,
) {
  return getJson<RevisionResponse>(base(id) + "/revisions/" + revision, signal);
}
export function createMaterial(
  content: ContentInput,
  ownerInstitutionId: number | null,
  reason: string,
) {
  return mutateJson<{ materialId: number; revisionId: number }>(
    "/api/me/author-materials",
    "POST",
    {
      content,
      ownerInstitutionId,
      reason,
    },
  );
}
export function saveRevision(
  revision: Revision,
  content: ContentInput,
  reason: string,
) {
  return mutateJson<{ version: number }>(
    base(revision.materialId) + "/revisions/" + revision.id,
    "PUT",
    {
      expectedVersion: revision.version,
      content,
      reason,
    },
  );
}
export function forkRevision(
  material: MaterialHeader,
  sourceRevisionId: number | null,
  reason: string,
) {
  return mutateJson<{ revisionId: number }>(
    base(material.id) + "/drafts",
    "POST",
    {
      expectedVersion: material.version,
      sourceRevisionId,
      reason,
    },
  );
}
export function transitionRevision(
  revision: Revision,
  action: "submit" | "withdraw" | "approve" | "reject" | "discard",
  reason: string,
) {
  return mutateJson(
    base(revision.materialId) + "/revisions/" + revision.id + "/transition",
    "POST",
    {
      expectedVersion: revision.version,
      action,
      reason,
    },
  );
}
export function changeMaterialState(
  material: MaterialHeader,
  action: "archive" | "restore" | "unpublish" | "republish" | "private",
  reason: string,
) {
  return mutateJson(base(material.id) + "/state", "POST", {
    expectedVersion: material.version,
    action,
    reason,
  });
}
export function restrictMaterial(
  material: MaterialHeader,
  institutionId: number | null,
  reason: string,
) {
  return mutateJson(base(material.id) + "/restrictions", "POST", {
    expectedVersion: material.version,
    institutionId,
    reason,
  });
}
export function unrestrictMaterial(
  material: MaterialHeader,
  restriction: number,
  reason: string,
) {
  return mutateJson(
    base(material.id) + "/restrictions/" + restriction + "/revoke",
    "POST",
    {
      expectedVersion: material.version,
      reason,
    },
  );
}
export function getCollaborators(
  id: number,
  query: AdminQuery,
  signal?: AbortSignal,
) {
  return getJson<Page<Collaborator>>(
    base(id) + "/collaborators?" + params(query),
    signal,
  );
}
export function setCollaborator(
  material: MaterialHeader,
  user: number,
  active: boolean,
  permissions: string[],
  reason: string,
) {
  return mutateJson(base(material.id) + "/collaborators/" + user, "PUT", {
    expectedVersion: material.version,
    active,
    permissions,
    reason,
  });
}
export function getMaterialEvents(
  id: number,
  query: AdminQuery,
  signal?: AbortSignal,
) {
  return getJson<Page<MaterialEvent>>(
    base(id) + "/events?" + params(query),
    signal,
  );
}
export function detachPlacement(
  material: MaterialHeader,
  placement: number,
  reason: string,
) {
  return mutateJson(
    base(material.id) + "/placements/" + placement + "/detach",
    "POST",
    {
      expectedVersion: material.version,
      reason,
    },
  );
}
export function getPlacementEntries(
  id: number,
  placement: number,
  page: number,
  signal?: AbortSignal,
) {
  return getJson<Page<PlacementEntry>>(
    base(id) + "/placements/" + placement + "/entries?page=" + page,
    signal,
  );
}
export function detachEntry(
  material: MaterialHeader,
  entry: number,
  reason: string,
) {
  return mutateJson(
    base(material.id) + "/entries/" + entry + "/detach",
    "POST",
    {
      expectedVersion: material.version,
      reason,
    },
  );
}
export function getPublishedHistory(
  id: number,
  page: number,
  authenticated: boolean,
  context: string,
  signal?: AbortSignal,
) {
  const prefix = authenticated ? "/api/me/materials/" : "/api/materials/";
  return getJson<PublishedHistory>(
    prefix + id + "/history?page=" + page + context,
    signal,
  );
}
export function getPublishedRevision(
  id: number,
  revision: number,
  authenticated: boolean,
  context: string,
  signal?: AbortSignal,
) {
  const prefix = authenticated ? "/api/me/materials/" : "/api/materials/";
  return getJson<RevisionResponse>(
    prefix + id + "/history/" + revision + "?" + context.replace(/^&/, ""),
    signal,
  );
}
