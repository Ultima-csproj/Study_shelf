import { getJson, mutateJson } from "./http";
import type { Page } from "./library";
export interface AdminAccess {
  permissions: string[];
  institutions: { id: number; name: string; permissions: string[] }[];
}
export interface AdminOverview {
  accounts: number | null;
  institutions: number | null;
  pendingMembers: number;
  pendingContributors: number;
  pendingInstitutions: number;
  pendingMaterials: number | null;
  memberAttention: {
    institutionId: number;
    institutionName: string;
    pendingCount: number;
  }[];
}
export interface AdminQuery {
  page: number;
  q: string;
  state: string;
}
export interface AdminUser {
  id: number;
  name: string;
  state: "active" | "disabled";
  version: number;
  email: string | null;
  permissions: string[];
}
export interface Membership {
  userId: number;
  institutionId: number;
  userName: string;
  institutionName: string;
  state: string;
  version: number;
  roles: string[];
  permissions: string[];
}
export interface UserDetail {
  user: AdminUser;
  memberships: Membership[];
  contributors: { id: number; name: string }[];
}
export interface Institution {
  id: number;
  name: string;
  version: number;
  memberCount: number;
  managerCount: number;
}
export interface AdminRequest {
  id: number;
  userId: number;
  userName: string;
  name: string;
  description: string;
  evidence: string | null;
  state: string;
  version: number;
  createdAt: string;
  institutionId: number | null;
  decisionReason: string | null;
}
export interface Audit {
  id: number;
  actorUserId: number | null;
  actorName: string | null;
  targetKind: string;
  targetId: number;
  institutionId: number | null;
  action: string;
  reason: string;
  beforeData: unknown;
  afterData: unknown;
  source: string;
  createdAt: string;
}
export interface MemberInput {
  expectedVersion: number;
  state: string;
  roles: string[];
  permissions: string[];
  reason: string;
}
function params(p: AdminQuery) {
  const query = new URLSearchParams({ page: String(p.page), pageSize: "20" });
  if (p.q) {
    query.set("q", p.q);
  }
  if (p.state) {
    query.set("state", p.state);
  }
  return query;
}
export function getAdminAccess(signal?: AbortSignal) {
  return getJson<AdminAccess>("/api/admin/access", signal);
}
export function getAdminOverview(signal?: AbortSignal) {
  return getJson<AdminOverview>("/api/admin/overview", signal);
}
export function getUsers(p: AdminQuery, signal?: AbortSignal) {
  return getJson<Page<AdminUser>>("/api/admin/users?" + params(p), signal);
}
export function getUser(id: number, signal?: AbortSignal) {
  return getJson<UserDetail>(`/api/admin/users/${id}`, signal);
}
export function setUserState(user: AdminUser, active: boolean, reason: string) {
  return mutateJson(`/api/admin/users/${user.id}/state`, "POST", {
    expectedVersion: user.version,
    active,
    reason,
  });
}
export function revokeUserSessions(id: number, reason: string) {
  return mutateJson(`/api/admin/users/${id}/sessions/revoke`, "POST", {
    reason,
  });
}
export function setPlatformPermissions(
  user: AdminUser,
  permissions: string[],
  reason: string,
) {
  return mutateJson(`/api/admin/users/${user.id}/permissions`, "PUT", {
    expectedVersion: user.version,
    permissions,
    reason,
  });
}
export function getInstitutions(p: AdminQuery, signal?: AbortSignal) {
  return getJson<Page<Institution>>(
    "/api/admin/institutions?" + params(p),
    signal,
  );
}
export function getInstitution(id: number, signal?: AbortSignal) {
  return getJson<Institution>(`/api/admin/institutions/${id}`, signal);
}
export function renameInstitution(
  item: Institution,
  name: string,
  reason: string,
) {
  return mutateJson(`/api/admin/institutions/${item.id}`, "PATCH", {
    expectedVersion: item.version,
    name,
    reason,
  });
}
export function getMembers(id: number, p: AdminQuery, signal?: AbortSignal) {
  return getJson<Page<Membership>>(
    `/api/admin/institutions/${id}/members?${params(p)}`,
    signal,
  );
}
export function setMembership(
  institution: number,
  user: number,
  input: MemberInput,
) {
  return mutateJson(
    `/api/admin/institutions/${institution}/members/${user}`,
    "PUT",
    input,
  );
}
export function decideMembership(
  m: Membership,
  accept: boolean,
  role: string,
  reason: string,
) {
  return mutateJson(
    `/api/admin/institutions/${m.institutionId}/requests/${m.userId}/decision`,
    "POST",
    { expectedVersion: m.version, accept, role, reason },
  );
}
export function getContributorRequests(p: AdminQuery, signal?: AbortSignal) {
  return getJson<Page<AdminRequest>>(
    "/api/admin/requests/contributors?" + params(p),
    signal,
  );
}
export function getInstitutionRequests(p: AdminQuery, signal?: AbortSignal) {
  return getJson<Page<AdminRequest>>(
    "/api/admin/requests/institutions?" + params(p),
    signal,
  );
}
export function decideRequest(
  kind: "contributors" | "institutions",
  r: AdminRequest,
  accept: boolean,
  reason: string,
) {
  const target = kind === "contributors" ? `${r.id}/${r.userId}` : String(r.id);
  return mutateJson(`/api/admin/requests/${kind}/${target}/decision`, "POST", {
    expectedVersion: r.version,
    accept,
    reason,
  });
}
export function getAudit(p: AdminQuery, signal?: AbortSignal) {
  return getJson<Page<Audit>>("/api/admin/audit?" + params(p), signal);
}
export function registerInstitution(
  name: string,
  description: string,
  evidence: string,
) {
  return mutateJson("/api/me/institution-requests", "POST", {
    name,
    description,
    evidence,
  });
}
export function getOwnInstitutionRequests(p: AdminQuery, signal?: AbortSignal) {
  return getJson<Page<AdminRequest>>(
    "/api/me/institution-requests?" + params(p),
    signal,
  );
}
export const platformLabels: Record<string, string> = {
  moderate_public_materials: "Модерация публикаций",
  manage_accounts: "Управление аккаунтами",
  manage_institutions: "Управление учреждениями",
  manage_platform_permissions: "Назначение глобальных разрешений",
  view_account_private_data: "Просмотр персональных данных аккаунтов",
  restrict_public_materials: "Ограничение публичных материалов платформы",
  review_restricted_materials: "Просмотр ограниченных публичных материалов",
};
export const memberLabels: Record<string, string> = {
  manage_members: "Управление участниками",
  manage_disciplines: "Управление дисциплинами",
  create_materials: "Создание материалов учреждения",
  manage_materials: "Управление материалами учреждения",
  restrict_materials: "Ограничение материалов в учреждении",
  review_restricted_materials: "Просмотр материалов с ограничением учреждения",
};
export const stateLabels: Record<string, string> = {
  active: "Активен",
  disabled: "Отключён",
  pending: "На рассмотрении",
  accepted: "Принята",
  rejected: "Отклонена",
  revoked: "Отозвано",
};
