import type {
  MaterialSummary,
  CatalogRequest,
  CatalogResponse,
} from "../types/material";
import { getJson } from "./http";

type CatalogHttpResponse = Omit<CatalogResponse, "items"> & {
  items: MaterialSummary[];
};

export async function getCatalog(
  request: CatalogRequest,
  signal?: AbortSignal,
): Promise<CatalogResponse> {
  const { filters, sort, page, pageSize } = request;
  const params = new URLSearchParams({
    sort,
    page: String(page),
    pageSize: String(pageSize),
  });

  if (filters.query.trim()) {
    params.set("q", filters.query.trim());
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

  const response = await getJson<CatalogHttpResponse>(
    `/api/catalog?${params}`,
    signal,
  );

  return {
    ...response,
    items: response.items.map((material) => ({ ...material, source: "api" })),
  };
}
