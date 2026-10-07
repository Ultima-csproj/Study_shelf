import type { MaterialSummary, ResponsiblePerson } from "./material";
export interface ApiPage<T> {
  items: T[];
  total: number;
  page: number;
  pageCount: number;
}
export interface LearningEntry {
  id: number;
  materialId: number;
  placementId: number | null;
  position: number;
  responsible: ResponsiblePerson | null;
  course: number | null;
}
export interface LearningSection {
  id: number;
  title: string;
  description: string;
  position: number;
  stepCount: number;
  items: LearningEntry[];
}
export interface LearningDetail {
  sections: LearningSection[];
  materials: MaterialSummary[];
  initialPageSize: number;
}
export interface SectionPage extends ApiPage<LearningEntry> {
  materials: MaterialSummary[];
}
