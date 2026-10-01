import type { CatalogMaterial } from "../types/material";
import type { CollectionDetail } from "../types/collection";
import { navigationOrigin, type NavigationOrigin } from "./navigation";

export interface NavigationCrumb {
  label: string;
  path: string;
}

export interface MaterialNavigation {
  backPath: string;
  backLabel: string;
  crumbs: NavigationCrumb[];
}

export function materialNavigation(
  origin: NavigationOrigin | null,
  material: CatalogMaterial | null,
  collection: CollectionDetail | null,
): MaterialNavigation {
  if (origin?.kind === "catalog") {
    return {
      backPath: origin.path,
      backLabel: "Вернуться в каталог",
      crumbs: [{ label: "Каталог", path: origin.path }],
    };
  }

  if (origin?.kind === "library") {
    return {
      backPath: origin.path,
      backLabel: "Вернуться в библиотеку",
      crumbs: [{ label: "Моя библиотека", path: origin.path }],
    };
  }

  if (origin?.kind === "collections") {
    return {
      backPath: origin.path,
      backLabel: "Вернуться к подборкам",
      crumbs: [{ label: "Подборки", path: origin.path }],
    };
  }

  const discipline = material?.discipline;
  const sameDiscipline =
    origin?.kind === "discipline" &&
    discipline &&
    (origin.id === discipline.slug || origin.id === String(discipline.id));

  if (origin?.kind === "discipline") {
    const disciplineLabel = sameDiscipline ? discipline.name : "Дисциплина";

    return {
      backPath: origin.path,
      backLabel: "Вернуться к дисциплине",
      crumbs: [
        { label: "Дисциплины", path: "/disciplines" },
        { label: disciplineLabel, path: origin.path },
      ],
    };
  }

  if (origin?.kind === "collection" && collection) {
    const collectionUrl = new URL(origin.path, "https://study-shelf.local");
    const parent = navigationOrigin(collectionUrl.search);
    const parentPath =
      parent?.kind === "library" || parent?.kind === "collections"
        ? parent.path
        : "/collections";
    const parentLabel =
      parent?.kind === "library" ? "Моя библиотека" : "Подборки";

    return {
      backPath: origin.path,
      backLabel: "Вернуться в подборку",
      crumbs: [
        { label: parentLabel, path: parentPath },
        { label: collection.title, path: origin.path },
      ],
    };
  }

  return {
    backPath: "/catalog",
    backLabel: "Вернуться в каталог",
    crumbs: [{ label: "Каталог", path: "/catalog" }],
  };
}
