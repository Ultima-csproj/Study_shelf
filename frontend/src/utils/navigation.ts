export type NavigationOrigin =
  | { kind: 'catalog' | 'collections' | 'library'; path: string }
  | { kind: 'discipline' | 'collection'; path: string; id: string }

export function parseNavigationOrigin(
  value: string | null,
): NavigationOrigin | null {
  if (
    !value ||
    !value.startsWith('/') ||
    value.startsWith('//') ||
    value.includes('\\')
  ) {
    return null
  }

  const url = new URL(value, 'https://study-shelf.local')

  if (url.origin !== 'https://study-shelf.local') {
    return null
  }

  const path = url.pathname + url.search + url.hash

  if (url.pathname === '/catalog') {
    return { kind: 'catalog', path }
  }

  if (url.pathname === '/collections') {
    return { kind: 'collections', path }
  }

  if (url.pathname === '/library') {
    return { kind: 'library', path }
  }

  const discipline = /^\/disciplines\/([a-z0-9-]+)$/.exec(url.pathname)

  if (discipline) {
    return { kind: 'discipline', path, id: discipline[1] }
  }

  const collection = /^\/collections\/([a-z0-9-]+)$/.exec(url.pathname)

  if (collection) {
    return { kind: 'collection', path, id: collection[1] }
  }

  return null
}

export function navigationOrigin(search: string): NavigationOrigin | null {
  const params = new URLSearchParams(search)
  return parseNavigationOrigin(params.get('from'))
}

export function locationPath(location: {
  pathname: string
  search: string
  hash: string
}): string {
  return location.pathname + location.search + location.hash
}

export function materialNavigationSection(location: {
  pathname: string
  search: string
}): 'catalog' | 'disciplines' | 'collections' | null {
  if (!location.pathname.startsWith('/materials/')) {
    return null
  }

  const origin = navigationOrigin(location.search)

  if (origin?.kind === 'collection' || origin?.kind === 'collections') {
    return 'collections'
  }

  if (origin?.kind === 'discipline') {
    return 'disciplines'
  }

  if (origin?.kind === 'library') {
    return null
  }

  return 'catalog'
}

export function withNavigationOrigin(path: string, parentPath: string): string {
  const origin = parseNavigationOrigin(parentPath)

  if (!origin) {
    return path
  }

  const params = new URLSearchParams({ from: origin.path })
  return `${path}?${params.toString()}`
}

export function collectionsForDiscipline(disciplineId: number): string {
  const params = new URLSearchParams({
    all: '1',
    discipline: String(disciplineId),
  })
  return `/collections?${params.toString()}`
}
