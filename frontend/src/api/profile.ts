import type { ProfileInput, User } from "../types/viewer";
import { getJson, mutateJson } from "./http";

export function updateProfile(
  input: ProfileInput,
  signal?: AbortSignal,
): Promise<User> {
  return mutateJson("/api/me/profile", "PATCH", input, signal);
}
export interface IdentityRequest {
  id: number;
  name: string;
  state: string;
}
export function getIdentityRequests(signal?: AbortSignal) {
  return getJson<{
    memberships: IdentityRequest[];
    contributors: IdentityRequest[];
  }>("/api/me/identity-requests", signal);
}
export function requestMembership(id: number) {
  return mutateJson(`/api/me/membership-requests/${id}`, "PUT");
}
export function requestContributor(id: number) {
  return mutateJson(`/api/me/contributor-requests/${id}`, "PUT");
}
