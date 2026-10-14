import type {
  CatalogFilters,
  CatalogRequest,
  MaterialType,
} from "../types/material";
const types: MaterialType[] = ["lecture", "book", "article", "video", "lab"];
const numbers = (params: URLSearchParams, key: string) => [
  ...new Set(
    params
      .getAll(key)
      .map(Number)
      .filter((value) => Number.isInteger(value) && value > 0),
  ),
];
export function readCatalogParams(params: URLSearchParams): CatalogRequest {
  const sort = params.get("sort");
  const disciplineIds = numbers(params, "discipline");
  return {
    filters: {
      includeArchived: params.get("includeArchived") === "true",
      query: params.get("q") ?? "",
      types: types.filter((type) => params.getAll("type").includes(type)),
      disciplineIds,
      teacherIds: numbers(params, "teacher"),
      courses: numbers(params, "course"),
      years: numbers(params, "year"),
      tags: [...new Set(params.getAll("tag"))],
    },
    sort: sort === "oldest" || sort === "title" ? sort : "newest",
    page: Math.max(1, Number.parseInt(params.get("page") ?? "1", 10) || 1),
    pageSize: 5,
  };
}
export function writeCatalogParams(
  filters: CatalogFilters,
  sort: CatalogRequest["sort"],
  page = 1,
) {
  const params = new URLSearchParams();
  if (filters.includeArchived) {
    params.set("includeArchived", "true");
  }
  if (filters.query) {
    params.set("q", filters.query);
  }
  if (!filters.disciplineIds.length) {
    params.set("all", "1");
  }
  for (const [key, values] of [
    ["type", filters.types],
    ["discipline", filters.disciplineIds],
    ["teacher", filters.teacherIds],
    ["course", filters.courses],
    ["year", filters.years],
    ["tag", filters.tags],
  ] as const) {
    for (const value of values) {
      params.append(key, String(value));
    }
  }
  if (sort !== "newest") {
    params.set("sort", sort);
  }
  if (page > 1) {
    params.set("page", String(page));
  }
  return params;
}
