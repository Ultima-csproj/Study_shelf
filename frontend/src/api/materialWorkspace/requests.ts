import type { AdminQuery } from "../admin";

export function params(query: AdminQuery) {
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

export const base = (id: number) => "/api/me/materials/" + id;
