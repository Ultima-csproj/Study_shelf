import type { CollectionRequest, CollectionSort } from "../types/collection";

function numericValues(params: URLSearchParams, name: string): number[] {
  return [...new Set(params.getAll(name).map(Number).filter((value) => {
    return Number.isInteger(value) && value > 0;
  }))];
}
export function readCollectionParams(params: URLSearchParams): CollectionRequest {
  const requestedSort = params.get("sort");
  const sort: CollectionSort =
    requestedSort === "title" || requestedSort === "oldest" ? requestedSort : "newest";
  return {
    query: params.get("q") ?? "",
    disciplineIds: numericValues(params, "discipline"),
    courses: numericValues(params, "course"),
    sort,
    page: Math.max(1, Number.parseInt(params.get("page") ?? "1", 10) || 1),
    pageSize: 10,
  };
}
export function writeCollectionParams(request: CollectionRequest): URLSearchParams {
  const params = new URLSearchParams();
  if (request.query) {
    params.set("q", request.query);
  }
  for (const id of request.disciplineIds) {
    params.append("discipline", String(id));
  }
  for (const course of request.courses) {
    params.append("course", String(course));
  }
  if (request.sort !== "newest") {
    params.set("sort", request.sort);
  }
  if (request.page > 1) {
    params.set("page", String(request.page));
  }
  return params;
}
