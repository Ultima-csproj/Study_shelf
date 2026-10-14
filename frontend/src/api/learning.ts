import type { CatalogMaterial, CourseMaterial, MaterialSummary } from "../types/material";
import type { LearningEntry } from "../types/learning";
import type { CollectionItemView } from "../types/collection";
import { authorLabel, materialTypeLabels } from "../utils/materialLabels";

export function materialMap(materials: MaterialSummary[]): Map<number, CatalogMaterial> {
  return new Map(materials.map((material) => {
    return [material.id, { ...material, source: "api" as const }];
  }));
}
function requiredMaterial(entry: LearningEntry, materials: Map<number, CatalogMaterial>): CatalogMaterial {
  const material = materials.get(entry.materialId);
  if (!material) {
    throw new Error("Сервер вернул неполные сведения о материале");
  }
  return material;
}
export function courseItem(entry: LearningEntry, materials: Map<number, CatalogMaterial>): CourseMaterial {
  const material = requiredMaterial(entry, materials);
  const authors = authorLabel(material.credits);
  let contributor = authors;
  if (entry.responsible) {
    contributor += " · Ответственный: " + entry.responsible.name;
  }
  if (entry.course !== null) {
    contributor += " · " + entry.course + " курс";
  }
  return {
    id: entry.id,
    material,
    number: entry.position,
    title: material.title,
    description: material.description,
    formatLabel: materialTypeLabels[material.type],
    contributor,
    addedAt: material.publishedAt,
    publicationYear: material.sourceYear ?? undefined,
  };
}
export function collectionItem(entry: LearningEntry, materials: Map<number, CatalogMaterial>): CollectionItemView {
  const material = requiredMaterial(entry, materials);
  return {
    id: entry.id,
    materialId: entry.materialId,
    position: entry.position,
    material,
    formatLabel: materialTypeLabels[material.type],
    attributionLabel: authorLabel(material.credits),
  };
}
