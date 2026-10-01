import type { MaterialReference } from "../types/material";

export function parseSavedMaterials(value: unknown): MaterialReference[] {
  if (!Array.isArray(value)) {
    return [];
  }

  const result: MaterialReference[] = [];

  for (const entry of value) {
    // Older demo sessions stored only numeric IDs, all from mock data.
    const reference: unknown =
      typeof entry === "number"
        ? { id: entry, source: "mock" }
        : entry;

    if (typeof reference !== "object" || reference === null) {
      continue;
    }

    if (!("id" in reference) || !("source" in reference)) {
      continue;
    }

    const { id, source } = reference;

    if (
      typeof id !== "number" ||
      !Number.isSafeInteger(id) ||
      id <= 0 ||
      (source !== "api" && source !== "mock")
    ) {
      continue;
    }

    if (!result.some((item) => item.id === id && item.source === source)) {
      result.push({ id, source });
    }
  }

  return result;
}
