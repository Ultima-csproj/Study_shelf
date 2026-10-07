import { clearCsrf, getJson, mutateJson } from "./http";
import type { ServerViewer } from "../types/viewer";

export interface LoginInput {
  email: string;
  password: string;
}
export interface RegisterInput extends LoginInput {
  firstName: string;
  lastName: string;
}
export function getViewer(signal?: AbortSignal) {
  return getJson<ServerViewer>("/api/viewer", signal);
}
export async function enter(
  input: LoginInput | RegisterInput,
  register: boolean,
) {
  const result = await mutateJson<ServerViewer>(
    register ? "/api/auth/register" : "/api/auth/login",
    "POST",
    input,
  );
  clearCsrf();
  return result;
}
export async function leave() {
  await mutateJson("/api/auth/logout", "POST");
  clearCsrf();
}
export async function changePassword(
  currentPassword: string,
  newPassword: string,
) {
  await mutateJson("/api/me/password", "POST", {
    currentPassword,
    newPassword,
  });
  clearCsrf();
}
export function logoutOthers() {
  return mutateJson("/api/auth/logout-others", "POST");
}
