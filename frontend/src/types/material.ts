
export type MaterialType = "lecture" | "book" | "article" | "video" | "lab";
export type CatalogSort = "newest" | "oldest" | "title";
export type MaterialSource = "api" | "mock";
export type ContributorRole = "author" | "co_author" | "editor" | "translator" | "lecturer" | "reviewer";

export interface MaterialReference {
  id: number;
  source: MaterialSource;
}
export interface MaterialCredit {
  contributorId: number;
  displayName: string;
  role: ContributorRole;
}
export interface EducationalDiscipline {
  id: number;
  slug: string;
  name: string;
}
export interface ResponsiblePerson {
  id: number;
  name: string;
  shortName: string;
}
export interface EducationalPlacement {
  id: number;
  discipline: EducationalDiscipline;
  responsible: ResponsiblePerson | null;
  course: number | null;
}
export interface MaterialSummary {
  id: number;
  publishedRevisionId: number;
  title: string;
  description: string;
  type: MaterialType;
  tags: string[];
  credits: MaterialCredit[];
  placements: EducationalPlacement[];
  sourceYear: number | null;
  publishedAt: string;
  firstPublishedAt: string;
  preview: string | null;
  durationSeconds: number | null;
  fileSizeBytes: number | null;
}
export interface MaterialDetail extends MaterialSummary {
  // File/body access is introduced in its own backend stage.
}
export interface CreditView extends Omit<MaterialCredit, "contributorId"> {
  contributorId: number | null;
}
export interface CatalogMaterial extends MaterialSummary {
  source: "api";
}

export type PreviewKind = "presentation" | "document" | "pdf";
export interface CourseMaterial {
  id: number;
  material: CatalogMaterial;
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
  id: number;
  position: number;
  stepCount: number;
  initialPageSize: number;
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
export interface CatalogFacets {
  types: Partial<Record<MaterialType, number>>;
  disciplines: (EducationalDiscipline & { materialCount: number })[];
  teachers: (ResponsiblePerson & { materialCount: number })[];
  courses: { value: number; materialCount: number }[];
  years: { value: number; materialCount: number }[];
}
