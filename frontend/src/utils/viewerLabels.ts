import type { UserRole } from '../types/viewer'

export const roleLabels: Record<UserRole, string> = {
  student: 'Студент',
  teacher: 'Преподаватель',
  admin: 'Администратор',
}
