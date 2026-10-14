import type { MaterialSource } from "../types/material";

export type NavigationOrigin =
  | {
      kind:
        | "catalog"
        | "disciplines"
        | "collections"
        | "library"
        | "profile"
        | "settings"
        | "home"
        | "author"
        | "workspace"
        | "material"
        | "personalCollections";
      path: string;
    }
  | { kind: "discipline" | "collection"; path: string; id: string };

export function parseNavigationOrigin(
  value: string | null,
): NavigationOrigin | null {
  if (
    !value ||
    !value.startsWith("/") ||
    value.startsWith("//") ||
    value.includes("\\") ||
    Array.from(value).some((character) => {
      const code = character.charCodeAt(0);
      return code < 32 || code === 127;
    })
  ) {
    return null;
  }

  const url = new URL(value, "https://study-shelf.local");

  if (url.origin !== "https://study-shelf.local") {
    return null;
  }

  const path = url.pathname + url.search + url.hash;

  if (url.pathname === "/catalog") {
    return { kind: "catalog", path };
  }

  if (url.pathname === "/collections") {
    return { kind: "collections", path };
  }
  if (url.pathname === "/disciplines") {
    return { kind: "disciplines", path };
  }

  if (url.pathname === "/library") {
    return { kind: "library", path };
  }

  if (url.pathname === "/profile") {
    return { kind: "profile", path };
  }

  if (url.pathname === "/settings") {
    return { kind: "settings", path };
  }
  if (url.pathname === "/") {
    return { kind: "home", path };
  }
  if (url.pathname === "/author/materials") {
    return { kind: "author", path };
  }
  if (/^\/author\/materials\/\d+$/.test(url.pathname)) {
    return { kind: "workspace", path };
  }
  if (/^\/materials\/\d+(?:\/history(?:\/\d+)?)?$/.test(url.pathname)) {
    return { kind: "material", path };
  }
  if (/^\/library\/collections(?:\/\d+)?$/.test(url.pathname)) {
    return { kind: "personalCollections", path };
  }

  const discipline = /^\/disciplines\/([a-z0-9-]+)$/.exec(url.pathname);

  if (discipline) {
    return { kind: "discipline", path, id: discipline[1] };
  }

  const collection = /^\/collections\/([a-z0-9-]+)$/.exec(url.pathname);

  if (collection) {
    return { kind: "collection", path, id: collection[1] };
  }

  return null;
}

export function navigationOrigin(search: string): NavigationOrigin | null {
  const params = new URLSearchParams(search);
  return parseNavigationOrigin(params.get("from"));
}

export function locationPath(location: {
  pathname: string;
  search: string;
  hash: string;
}): string {
  return location.pathname + location.search + location.hash;
}

export function materialNavigationSection(location: {
  pathname: string;
  search: string;
}): "catalog" | "disciplines" | "collections" | null {
  if (!location.pathname.startsWith("/materials/")) {
    return null;
  }

  const origin = navigationOrigin(location.search);

  if (origin?.kind === "collection" || origin?.kind === "collections") {
    return "collections";
  }

  if (origin?.kind === "discipline" || origin?.kind === "disciplines") {
    return "disciplines";
  }

  if (origin && origin.kind !== "catalog") {
    return null;
  }

  return "catalog";
}

export function navigationLabel(origin: NavigationOrigin): string {
  if (origin.kind === "profile") {
    const url = new URL(origin.path, "https://study-shelf.local");
    return url.searchParams.get("tab") === "publications"
      ? "Публикации"
      : "Обзор профиля";
  }
  const labels: Record<Exclude<NavigationOrigin["kind"], "profile">, string> = {
    catalog: "Каталог",
    disciplines: "Дисциплины",
    collections: "Подборки",
    library: "Библиотека",
    settings: "Настройки",
    home: "Главная",
    author: "Мои материалы",
    workspace: "Рабочая область",
    material: "Материал",
    personalCollections: "Мои подборки",
    discipline: "Дисциплина",
    collection: "Подборка",
  };
  return labels[origin.kind];
}

export function returnDestination(search: string, fallback: string) {
  const origin = navigationOrigin(search) ?? parseNavigationOrigin(fallback);
  return {
    path: origin?.path ?? "/profile",
    label: origin ? navigationLabel(origin) : "Обзор профиля",
  };
}

export function withNavigationOrigin(path: string, parentPath: string): string {
  const origin = parseNavigationOrigin(parentPath);

  if (!origin) {
    return path;
  }

  const url = new URL(path, "https://study-shelf.local");
  url.searchParams.set("from", origin.path);
  return url.pathname + url.search + url.hash;
}

export function authorListPath(location: {
  pathname: string;
  search: string;
  hash: string;
}): string {
  if (location.pathname === "/author/materials") {
    return locationPath(location);
  }
  if (location.pathname.startsWith("/author/materials/")) {
    const origin = navigationOrigin(location.search);
    if (origin?.kind === "author") {
      return origin.path;
    }
    return withNavigationOrigin(
      "/author/materials",
      origin?.path ?? "/profile",
    );
  }
  return withNavigationOrigin("/author/materials", locationPath(location));
}

export function materialPath(id: number, source: MaterialSource): string {
  const path = `/materials/${id}`;

  if (source === "mock") {
    return `${path}?source=mock`;
  }

  return path;
}

export function materialSource(search: string): MaterialSource {
  return new URLSearchParams(search).get("source") === "mock" ? "mock" : "api";
}

export function collectionsForDiscipline(disciplineId: number): string {
  const params = new URLSearchParams({
    all: "1",
    discipline: String(disciplineId),
  });
  return `/collections?${params.toString()}`;
}
