export interface User {
  id: number;
  name: string;
  initials: string;
  firstName: string;
  lastName: string;
  role: UserRole;
  course: number | null;
  group: string;
}
export type UserRole = "student" | "teacher" | "admin";
export type Viewer = { kind: "guest" } | { kind: "authenticated"; user: User };

export interface ProfileInput {
  firstName: string;
  lastName: string;
  course: number | null;
  group: string;
}
