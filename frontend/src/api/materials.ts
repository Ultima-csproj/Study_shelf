import { materials } from '../data/materials'
import { collectionMaterials } from '../data/collectionMaterials'
import { disciplines } from '../data/disciplines'
import { teachers } from '../data/teachers'
import type { CatalogMaterial } from '../types/material'

export async function getMaterial(
  id: string | number,
  signal?: AbortSignal,
): Promise<CatalogMaterial | null> {
  await Promise.resolve()
  signal?.throwIfAborted()
  const material = [...materials, ...collectionMaterials].find(
    (item) => item.id === Number(id),
  )
  return material
    ? {
        ...material,
        discipline:
          disciplines.find((item) => item.id === material.disciplineId) ?? null,
        teacher:
          teachers.find((item) => item.id === material.teacherId) ?? null,
      }
    : null
}
