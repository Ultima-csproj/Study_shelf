import { navigationOrigin } from "./navigation";

export function materialContext(search: string): string {
  const current = new URLSearchParams(search);
  const restriction = current.get("restrictionId");
  if (restriction) {
    return "&restrictionId=" + encodeURIComponent(restriction);
  }
  const origin = navigationOrigin(search);
  if (origin?.kind === "discipline") {
    return "&disciplineId=" + encodeURIComponent(origin.id);
  }
  if (origin?.kind === "collection") {
    return "&collectionId=" + encodeURIComponent(origin.id);
  }
  return "";
}
