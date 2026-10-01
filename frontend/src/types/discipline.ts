export interface Discipline {
  id: number;
  name: string;
  course: number;
  slug: string;
  semester: number;
  teacherId: number;
  description: string;
  summary?: string;
}
export interface DisciplineSummary extends Discipline {
  materialCount: number;
  teacher: Teacher | null;
}
import type { Teacher } from "./teacher";
