import { getJson } from "../http";
import type { RevisionResponse, PublishedHistory } from "./types";

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
