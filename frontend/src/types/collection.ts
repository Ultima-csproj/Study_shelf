import type { Discipline } from "./discipline";
import type { CatalogMaterial } from "./material";

export type CollectionRole = "author" | "curator" | "contributor";

export type ContributorIdentity =
  | { kind: "teacher"; teacherId: number; displayName: string }
  | { kind: "user"; userId: number; displayName: string }
  | { kind: "organization"; organizationId: number; displayName: string };

export interface CollectionContributor {
  identity: ContributorIdentity;
  role: CollectionRole;
  position: number;
}

export interface CollectionItem {
  id: number;
  materialId: number;
  position: number;
}

export interface CollectionSection {
  id: string;
  title: string;
  position: number;
  items: CollectionItem[];
}

export interface Collection {
  id: number;
  slug: string;
  title: string;
  shortDescription: string;
  description: string;
  purpose: string;
  targetAudience: string;
  estimatedTime: string | null;
  difficulty: "beginner" | "intermediate" | "advanced" | null;
  disciplineId: number | null;
  course: number | null;
  isPublic: boolean;
  createdAt: string;
  updatedAt: string;
  contributors: CollectionContributor[];
  sections: CollectionSection[];
}

export interface CollectionSummary extends Omit<Collection, "sections"> {
  discipline: Discipline | null;
  materialCount: number;
}

export interface CollectionItemView extends CollectionItem {
  material: CatalogMaterial;
  formatLabel: string;
  attributionLabel: string;
}

export interface CollectionSectionView extends Omit<
  CollectionSection,
  "items"
> {
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
}

export interface CollectionResponse {
  items: CollectionSummary[];
  total: number;
  disciplines: Discipline[];
  courses: number[];
}
