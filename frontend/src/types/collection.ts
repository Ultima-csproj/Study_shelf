import type { EducationalDiscipline, CatalogMaterial } from "./material";
import type { ApiPage } from "./learning";

export type CollectionRole = "author" | "curator" | "contributor";
export interface CollectionContributor {
  contributorId: number;
  displayName: string;
  role: CollectionRole;
  position: number;
}
export interface CollectionSummary {
  id: number;
  slug: string;
  title: string;
  shortDescription: string;
  description: string;
  purpose: string;
  targetAudience: string;
  discipline: EducationalDiscipline | null;
  course: number | null;
  difficulty: "beginner" | "intermediate" | "advanced" | null;
  estimatedMinutes: number | null;
  createdAt: string;
  updatedAt: string;
  contributors: CollectionContributor[];
  stepCount: number;
  uniqueMaterialCount: number;
}
export interface CollectionItemView {
  id: number;
  materialId: number;
  position: number;
  material: CatalogMaterial;
  formatLabel: string;
  attributionLabel: string;
}
export interface CollectionSectionView {
  id: number;
  title: string;
  description: string;
  position: number;
  stepCount: number;
  initialPageSize: number;
  items: CollectionItemView[];
}
export interface CollectionDetail extends CollectionSummary {
  sections: CollectionSectionView[];
}
export type CollectionSort = "newest" | "oldest" | "title";
export interface CollectionRequest {
  query: string;
  disciplineIds: number[];
  courses: number[];
  sort: CollectionSort;
  page: number;
  pageSize: number;
}
export interface CollectionResponse extends ApiPage<CollectionSummary> {
  disciplines: EducationalDiscipline[];
  courses: number[];
}
