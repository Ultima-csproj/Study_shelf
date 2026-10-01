export type MaterialType = "lecture" | "book" | "article" | "video" | "lab";
export type CatalogSort = "newest" | "oldest" | "title";
export type MaterialSource = "api" | "mock";

export interface MaterialReference {
  id: number;
  source: MaterialSource;
}

export interface Material {
  id: number;
  title: string;
  description: string;
  type: MaterialType;
  disciplineId: number;
  teacherId: number;
  tags: string[];
  year: number;
  course: number;
  createdAt: string;
  dateLabel: "Добавлено" | "Обновлено";
  preview?: string;
  duration?: string;
  fileSize?: string;
  authorName?: string;
}

export type PreviewKind = "presentation" | "document" | "pdf";
export interface CourseMaterial {
  material: Material;
  number: number;
  title: string;
  description: string;
  summary?: string;
  formatLabel: string;
  contributor: string;
  addedAt: string;
  fileSize?: string;
  publicationYear?: number;
  previewKind?: PreviewKind;
}
export interface MaterialSectionData {
  id: "start" | "lectures" | "practice";
  title: string;
  description: string;
  mobileDescription?: string;
  items: CourseMaterial[];
}

export interface CatalogFilters {
  query: string;
  types: MaterialType[];
  disciplineIds: number[];
  teacherIds: number[];
  courses: number[];
  years: number[];
  tags: string[];
}

export interface CatalogRequest {
  filters: CatalogFilters;
  sort: CatalogSort;
  page: number;
  pageSize: number;
}

export interface CatalogResponse {
  items: CatalogMaterial[];
  total: number;
  page: number;
  pageCount: number;
  facets: CatalogFacets;
}
import type { Discipline } from "./discipline";
import type { Teacher } from "./teacher";

export interface CatalogMaterial extends Material {
  source: MaterialSource;
  discipline: Discipline | null;
  teacher: Teacher | null;
}
export interface CatalogFacets {
  types: Partial<Record<MaterialType, number>>;
  disciplines: (Discipline & { materialCount: number })[];
  teachers: (Teacher & { materialCount: number })[];
  courses: { value: number; materialCount: number }[];
  years: { value: number; materialCount: number }[];
}
