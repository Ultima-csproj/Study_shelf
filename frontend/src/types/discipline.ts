import type { ResponsiblePerson } from "./material";
export interface Discipline {
  id: number;
  name: string;
  slug: string;
  description: string;
  summary: string;
  institution: { id: number; name: string };
  materialCount: number;
  stepCount: number;
  courses: number[];
  responsible: ResponsiblePerson[];
}
export type DisciplineSummary = Discipline;
