import type { UserRole } from "../types/viewer";

export const roleLabels: Record<UserRole, string> = {
  user: "Пользователь",
  student: "Студент",
  teacher: "Преподаватель",
  admin: "Администратор",
};
