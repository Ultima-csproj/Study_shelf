import { mutateJson } from "../http";
import type { MaterialHeader, ContentInput, Revision } from "./types";
import { base } from "./requests";

export function createMaterial(
  content: ContentInput,
  ownerInstitutionId: number | null,
) {
  return mutateJson<{ materialId: number; revisionId: number }>(
    "/api/me/author-materials",
    "POST",
    {
      content,
      ownerInstitutionId,
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
