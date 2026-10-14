import { getJson } from "../http";
import type { Page } from "../library";
import type { AdminQuery } from "../admin";
import type {
  MaterialHeader,
  Restriction,
  RestrictionTarget,
  Workspace,
  RevisionResponse,
  Collaborator,
  MaterialEvent,
  AuthorQuery,
  ContributorChoice,
} from "./types";
import { base, params } from "./requests";

export function getAuthorMaterials(query: AuthorQuery, signal?: AbortSignal) {
  const search = params(query);
  search.set("order", query.order ?? "newest");
  return getJson<Page<MaterialHeader>>(
    "/api/me/author-materials?" + search,
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

export function findMaterialContributors(query: string, signal?: AbortSignal) {
  return getJson<ContributorChoice[]>(
    "/api/me/material-contributors?" + new URLSearchParams({ q: query }),
    signal,
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
