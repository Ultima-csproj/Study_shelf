import type { MaterialType } from "./material";

// Isolated fixtures for course/collection screens until their backend stage.
export interface DemoMaterial {
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
