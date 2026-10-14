import { mutateJson } from "../http";
import type { MaterialHeader } from "./types";
import { base } from "./requests";

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
