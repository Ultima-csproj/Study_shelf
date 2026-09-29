import { collections } from '../data/collections'
import { disciplines } from '../data/disciplines'
import { routeMaterialSources } from '../data/collectionMaterials'
import { getMaterial } from './materials'
import type {
  Collection,
  CollectionDetail,
  CollectionItemView,
  CollectionRequest,
  CollectionResponse,
  CollectionSectionView,
  CollectionSummary,
} from '../types/collection'

function summarize(collection: Collection): CollectionSummary {
  const { sections, ...details } = collection
  const materialCount = sections.reduce(
    (total, item) => total + item.items.length,
    0,
  )
  const discipline =
    disciplines.find((item) => item.id === collection.disciplineId) ?? null

  return { ...details, materialCount, discipline }
}

export async function getCollections(
  request: CollectionRequest,
  signal?: AbortSignal,
): Promise<CollectionResponse> {
  await Promise.resolve()
  signal?.throwIfAborted()

  const query = request.query.trim().toLocaleLowerCase('ru')
  const items = collections
    .filter((collection) => {
      if (!collection.isPublic) {
        return false
      }

      if (request.disciplineIds.length > 0) {
        if (
          collection.disciplineId === null ||
          !request.disciplineIds.includes(collection.disciplineId)
        ) {
          return false
        }
      }

      if (request.courses.length > 0) {
        if (
          collection.course === null ||
          !request.courses.includes(collection.course)
        ) {
          return false
        }
      }

      const searchText = [
        collection.title,
        collection.shortDescription,
        collection.purpose,
      ]
        .join(' ')
        .toLocaleLowerCase('ru')

      return searchText.includes(query)
    })
    .map(summarize)

  items.sort((left, right) => {
    if (request.sort === 'title') {
      return left.title.localeCompare(right.title, 'ru')
    }

    if (request.sort === 'oldest') {
      return left.createdAt.localeCompare(right.createdAt)
    }

    return right.createdAt.localeCompare(left.createdAt)
  })

  return {
    items,
    total: items.length,
    disciplines: [1, 2, 5].flatMap((id) => {
      const discipline = disciplines.find((item) => item.id === id)
      return discipline ? [discipline] : []
    }),
    courses: [1, 2, 3],
  }
}

export async function getCollection(
  id: string | number,
  signal?: AbortSignal,
): Promise<CollectionDetail | null> {
  await Promise.resolve()
  signal?.throwIfAborted()

  const collection = collections.find((item) => {
    return item.isPublic && (item.slug === String(id) || item.id === Number(id))
  })

  if (!collection) {
    return null
  }

  const orderedSections = [...collection.sections].sort(
    (left, right) => left.position - right.position,
  )
  const sections: CollectionSectionView[] = await Promise.all(
    orderedSections.map(async (section) => {
      const orderedItems = [...section.items].sort(
        (left, right) => left.position - right.position,
      )
      const items: CollectionItemView[] = await Promise.all(
        orderedItems.map(async (item) => {
          const material = await getMaterial(item.materialId, signal)

          if (!material) {
            throw new Error(
              'Один из материалов подборки недоступен. Повторите загрузку.',
            )
          }

          const source = routeMaterialSources.has(material.id)
          const name =
            material.authorName ?? material.teacher?.shortName ?? 'Не указан'
          const attributionLabel = `${source ? 'Источник' : 'Автор'}: ${name}`
          let formatLabel = 'Документ'

          if (material.type === 'video') {
            formatLabel = material.id === 1001 ? 'Видеолекция' : 'Видео'
          } else if (material.type === 'lab') {
            formatLabel = 'Практикум'
          } else if (material.type === 'article') {
            formatLabel = source ? 'Внешняя статья' : 'Статья'
          } else if (material.id === 1002) {
            formatLabel = 'Лекция'
          } else if (material.id === 1006) {
            formatLabel = 'Методичка'
          } else if (material.id === 1007) {
            formatLabel = 'Конспект'
          }

          return { ...item, material, formatLabel, attributionLabel }
        }),
      )

      return { ...section, items }
    }),
  )

  signal?.throwIfAborted()
  return { ...summarize(collection), sections }
}
