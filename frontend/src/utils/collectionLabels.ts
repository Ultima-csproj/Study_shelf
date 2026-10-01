import type {
  CollectionContributor,
  CollectionSummary,
} from "../types/collection";

export function compilerNames(contributors: CollectionContributor[]): string {
  return [...contributors]
    .sort((left, right) => left.position - right.position)
    .map((contributor) => contributor.identity.displayName)
    .join(", ");
}

export function collectionMaterialCount(count: number): string {
  const lastTwo = count % 100;
  const last = count % 10;

  if (lastTwo >= 11 && lastTwo <= 14) {
    return `${count} материалов`;
  }

  if (last === 1) {
    return `${count} материал`;
  }

  if (last >= 2 && last <= 4) {
    return `${count} материала`;
  }

  return `${count} материалов`;
}

export function collectionMetadata(collection: CollectionSummary): string {
  const parts = [collectionMaterialCount(collection.materialCount)];

  if (collection.discipline) {
    parts.push(collection.discipline.name);
  }

  if (collection.course !== null) {
    parts.push(`${collection.course} курс`);
  }

  if (collection.estimatedTime) {
    parts.push(collection.estimatedTime);
  }

  return parts.join(" · ");
}

export const difficultyLabels = {
  beginner: "Начальный уровень",
  intermediate: "Средняя сложность",
  advanced: "Высокая сложность",
};
