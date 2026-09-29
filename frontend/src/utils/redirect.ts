export function safeRedirect(value: string | null) {
  if (
    !value ||
    !value.startsWith('/') ||
    value.startsWith('//') ||
    value.includes('\\')
  ) {
    return '/catalog'
  }
  const url = new URL(value, 'https://study-shelf.local')
  if (url.origin !== 'https://study-shelf.local' || url.pathname === '/login') {
    return '/catalog'
  }
  return url.pathname + url.search + url.hash
}

export function loginPath(location: {
  pathname: string
  search: string
  hash: string
}) {
  const currentPath = location.pathname + location.search + location.hash
  const params = new URLSearchParams(location.search)
  const redirect =
    location.pathname === '/login'
      ? safeRedirect(params.get('redirect'))
      : currentPath

  return `/login?redirect=${encodeURIComponent(redirect)}`
}
