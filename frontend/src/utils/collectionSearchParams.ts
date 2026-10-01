import type { CollectionRequest, CollectionSort } from "../types/collection";

function numericValues(params: URLSearchParams, name: string): number[] {
  return params
    .getAll(name)
    .map(Number)
    .filter((value) => {
      return Number.isInteger(value) && value > 0;
    });
}

export function readCollectionParams(
  params: URLSearchParams,
): CollectionRequest {
  const requestedSort = params.get("sort");
  const sort: CollectionSort =
    requestedSort === "title" || requestedSort === "oldest"
      ? requestedSort
      : "newest";
  const defaultFilters = params.size === 0;

  return {
    query: params.get("q") ?? "",
    disciplineIds: defaultFilters ? [1] : numericValues(params, "discipline"),
    courses: defaultFilters ? [3] : numericValues(params, "course"),
    sort,
  };
}

export function writeCollectionParams(
  request: CollectionRequest,
): URLSearchParams {
  const params = new URLSearchParams();
  params.set("all", "1");

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

  return params;
}
