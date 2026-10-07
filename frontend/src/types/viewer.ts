export interface User {
  id: number;
  name: string;
  initials: string;
  firstName: string;
  lastName: string;
  role: UserRole;
  course: number | null;
  group: string;
  email: string;
  emailVerified: boolean;
  biography: string;
  primaryContributorId: number;
  permissions: string[];
  memberships: Membership[];
}
export interface Membership {
  institutionId: number;
  institutionName: string;
  state: "pending" | "active" | "rejected" | "revoked";
  roles: ("student" | "teacher")[];
  permissions: string[];
}
export type UserRole = "user" | "student" | "teacher" | "admin";
export type ServerViewer =
  | { kind: "guest" }
  | { kind: "authenticated"; user: User };
export type Viewer =
  | ServerViewer
  | { kind: "loading" }
  | { kind: "error"; message: string };

export interface ProfileInput {
  firstName: string;
  lastName: string;
  course: number | null;
  group: string;
  biography: string;
}
