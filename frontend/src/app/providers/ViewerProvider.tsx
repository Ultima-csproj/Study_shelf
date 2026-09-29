import { useEffect, useState, type ReactNode } from 'react'
import { ViewerContext } from './ViewerContext'
import { demoUser } from '../../data/viewer'
import type { User, Viewer } from '../../types/viewer'

const viewerKey = 'study-shelf-demo-viewer'
const materialsKey = 'study-shelf-demo-saved'
const collectionsKey = 'study-shelf-demo-collections'
const profileKey = 'study-shelf-demo-profile'

function initialSaved(storageKey: string): number[] {
  try {
    if (sessionStorage.getItem(viewerKey) !== 'authenticated') {
      return []
    }

    const value: unknown = JSON.parse(
      sessionStorage.getItem(storageKey) ?? '[]',
    )

    if (!Array.isArray(value)) {
      return []
    }

    return value.filter((id): id is number => {
      return typeof id === 'number' && Number.isInteger(id) && id > 0
    })
  } catch {
    return []
  }
}

function initialViewer(): Viewer {
  try {
    if (sessionStorage.getItem(viewerKey) === 'authenticated') {
      const stored: unknown = JSON.parse(
        sessionStorage.getItem(profileKey) ?? 'null',
      )

      if (typeof stored === 'object' && stored !== null) {
        const profile = stored as Partial<User>
        const firstName = profile.firstName
        const lastName = profile.lastName
        const course = profile.course
        const group = profile.group
        const validCourse =
          course === null ||
          (typeof course === 'number' &&
            Number.isInteger(course) &&
            course >= 1 &&
            course <= 6)

        if (
          typeof firstName === 'string' &&
          firstName.trim() &&
          firstName.length <= 64 &&
          typeof lastName === 'string' &&
          lastName.trim() &&
          lastName.length <= 64 &&
          typeof group === 'string' &&
          group.length <= 30 &&
          validCourse
        ) {
          return {
            kind: 'authenticated',
            user: {
              ...demoUser,
              firstName: firstName.trim(),
              lastName: lastName.trim(),
              name: `${firstName.trim()} ${lastName.trim()[0]}.`,
              initials:
                `${firstName.trim()[0]}${lastName.trim()[0]}`.toLocaleUpperCase(
                  'ru',
                ),
              course: course ?? null,
              group,
            },
          }
        }
      }

      return { kind: 'authenticated', user: demoUser }
    }
  } catch {
    // Browsing remains available without session storage.
  }

  return { kind: 'guest' }
}

export function ViewerProvider({ children }: { children: ReactNode }) {
  const [viewer, setViewer] = useState<Viewer>(initialViewer)
  const [savedMaterialIds, setSavedMaterialIds] = useState<number[]>(() =>
    initialSaved(materialsKey),
  )
  const [savedCollectionIds, setSavedCollectionIds] = useState<number[]>(() =>
    initialSaved(collectionsKey),
  )

  useEffect(() => {
    try {
      if (viewer.kind === 'authenticated') {
        sessionStorage.setItem(viewerKey, 'authenticated')
        sessionStorage.setItem(profileKey, JSON.stringify(viewer.user))
        sessionStorage.setItem(materialsKey, JSON.stringify(savedMaterialIds))
        sessionStorage.setItem(
          collectionsKey,
          JSON.stringify(savedCollectionIds),
        )
      } else {
        sessionStorage.removeItem(viewerKey)
        sessionStorage.removeItem(profileKey)
        sessionStorage.removeItem(materialsKey)
        sessionStorage.removeItem(collectionsKey)
      }
    } catch {
      // Demo persistence is optional; the current UI state still works.
    }
  }, [viewer, savedMaterialIds, savedCollectionIds])

  const signInDemo = () => {
    setViewer((current) => {
      if (current.kind === 'authenticated') {
        return current
      }

      return { kind: 'authenticated', user: demoUser }
    })
  }

  const signOut = () => {
    setViewer({ kind: 'guest' })
    setSavedMaterialIds([])
    setSavedCollectionIds([])
  }

  const toggleSaved = (id: number) => {
    if (viewer.kind === 'authenticated') {
      setSavedMaterialIds((ids) => {
        if (ids.includes(id)) {
          return ids.filter((value) => value !== id)
        }

        return [...ids, id]
      })
    }
  }

  const toggleSavedCollection = (id: number) => {
    if (viewer.kind === 'authenticated') {
      setSavedCollectionIds((ids) => {
        if (ids.includes(id)) {
          return ids.filter((value) => value !== id)
        }

        return [...ids, id]
      })
    }
  }

  const updateUser = (user: User) => {
    setViewer((current) => {
      if (current.kind !== 'authenticated' || current.user.id !== user.id) {
        return current
      }

      return { kind: 'authenticated', user }
    })
  }

  const value = {
    viewer,
    savedMaterialIds,
    savedCollectionIds,
    signInDemo,
    signOut,
    toggleSaved,
    toggleSavedCollection,
    updateUser,
  }

  return (
    <ViewerContext.Provider value={value}>{children}</ViewerContext.Provider>
  )
}
