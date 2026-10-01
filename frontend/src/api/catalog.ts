import { materials } from "../data/materials";
import { disciplines } from "../data/disciplines";
import { teachers } from "../data/teachers";
import type {
  CatalogFacets,
  CatalogRequest,
  CatalogResponse,
  MaterialType,
} from "../types/material";

const normalize = (value: string) =>
  value.toLocaleLowerCase("ru-RU").replaceAll("ё", "е");
const types: MaterialType[] = ["lecture", "book", "article", "video", "lab"];
const facets: CatalogFacets = {
  types: Object.fromEntries(
    types.map((type) => [
      type,
      materials.filter((item) => item.type === type).length,
    ]),
  ),
  disciplines: disciplines.map((discipline) => ({
    ...discipline,
    materialCount: materials.filter(
      (item) => item.disciplineId === discipline.id,
    ).length,
  })),
  teachers: teachers.map((teacher) => ({
    ...teacher,
    materialCount: materials.filter((item) => item.teacherId === teacher.id)
      .length,
  })),
  courses: [1, 2, 3, 4].map((value) => ({
    value,
    materialCount: materials.filter((item) => item.course === value).length,
  })),
  years: [2026, 2025, 2024, 2023].map((value) => ({
    value,
    materialCount: materials.filter((item) => item.year === value).length,
  })),
};

// The single mock API boundary. Later replace this function with an Axum REST request.
export async function getCatalog(
  request: CatalogRequest,
  signal?: AbortSignal,
): Promise<CatalogResponse> {
  await Promise.resolve();
  signal?.throwIfAborted();
  const { filters, sort, pageSize } = request;
  const words = normalize(filters.query.trim()).split(/\s+/).filter(Boolean);
  const result = materials
    .filter((material) => {
      const discipline = disciplines.find(
        (item) => item.id === material.disciplineId,
      );
      const teacher = teachers.find((item) => item.id === material.teacherId);
      const haystack = normalize(
        [
          material.title,
          material.description,
          discipline?.name,
          teacher?.name,
          ...material.tags,
        ].join(" "),
      );
      return (
        words.every((word) => haystack.includes(word)) &&
        (!filters.types.length || filters.types.includes(material.type)) &&
        (!filters.disciplineIds.length ||
          filters.disciplineIds.includes(material.disciplineId)) &&
        (!filters.teacherIds.length ||
          filters.teacherIds.includes(material.teacherId)) &&
        (!filters.courses.length ||
          filters.courses.includes(material.course)) &&
        (!filters.years.length || filters.years.includes(material.year)) &&
        filters.tags.every((tag) => material.tags.includes(tag))
      );
    })
    .sort((a, b) =>
      sort === "title"
        ? a.title.localeCompare(b.title, "ru")
        : (sort === "newest"
            ? b.createdAt.localeCompare(a.createdAt)
            : a.createdAt.localeCompare(b.createdAt)) || a.id - b.id,
    );
  const total = result.length;
  const pageCount = Math.max(1, Math.ceil(total / pageSize));
  const page = Math.max(1, Math.min(request.page, pageCount));
  const items = result
    .slice((page - 1) * pageSize, page * pageSize)
    .map((material) => ({
      ...material,
      discipline:
        disciplines.find((item) => item.id === material.disciplineId) ?? null,
      teacher: teachers.find((item) => item.id === material.teacherId) ?? null,
    }));
  return { items, total, page, pageCount, facets };
}
