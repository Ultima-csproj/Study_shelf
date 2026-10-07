import { getJson, HttpError } from "./http";
import { courseItem, materialMap } from "./learning";
import type { DisciplineSummary } from "../types/discipline";
import type { ApiPage, LearningDetail, SectionPage } from "../types/learning";
import type { MaterialSectionData } from "../types/material";

export async function getDisciplines(
  signal?: AbortSignal,
  page = 1,
  pageSize = 20,
): Promise<ApiPage<DisciplineSummary>> {
  return getJson(`/api/disciplines?page=${page}&pageSize=${pageSize}`, signal);
}
export async function getDiscipline(id: string | number, signal?: AbortSignal) {
  try {
    const response = await getJson<DisciplineSummary & LearningDetail>(
      "/api/disciplines/" + encodeURIComponent(String(id)),
      signal,
    );
    const materials = materialMap(response.materials);
    const sections: MaterialSectionData[] = response.sections.map((section) => {
      return {
        ...section,
        initialPageSize: response.initialPageSize,
        items: section.items.map((entry) => courseItem(entry, materials)),
      };
    });
    return { discipline: response, sections };
  } catch (reason) {
    if (reason instanceof HttpError && reason.status === 404) {
      return null;
    }
    throw reason;
  }
}
export async function getDisciplineSectionItems(
  id: number,
  section: number,
  page: number,
  pageSize: number,
  signal?: AbortSignal,
) {
  const response = await getJson<SectionPage>(
    `/api/disciplines/${id}/sections/${section}/items?page=${page}&pageSize=${pageSize}`,
    signal,
  );
  const materials = materialMap(response.materials);
  return {
    ...response,
    items: response.items.map((entry) => courseItem(entry, materials)),
  };
}
