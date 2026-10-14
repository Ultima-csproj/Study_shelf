import { getJson, mutateJson } from "../http";
import type { Page } from "../library";
import type { MaterialHeader, PlacementEntry } from "./types";
import { base } from "./requests";

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
