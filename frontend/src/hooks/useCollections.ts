import { useCallback } from 'react'
import { getCollection, getCollections } from '../api/collections'
import type { CollectionRequest } from '../types/collection'
import { useApiResource } from './useApiResource'

export function useCollections(request: CollectionRequest) {
  const load = useCallback(
    (signal: AbortSignal) => {
      return getCollections(request, signal)
    },
    [request],
  )

  return useApiResource(load, 'Не удалось загрузить подборки')
}

export function useCollection(id: string | null) {
  const load = useCallback(
    (signal: AbortSignal) => {
      if (id === null) {
        return Promise.resolve(null)
      }

      return getCollection(id, signal)
    },
    [id],
  )

  return useApiResource(load, 'Не удалось загрузить подборку')
}
