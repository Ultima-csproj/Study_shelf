import { useEffect } from 'react'
import { Outlet, useLocation, useNavigate } from 'react-router-dom'
import { Header } from '../components/layout/Header/Header'
export function AppShell() {
  const location = useLocation()
  const navigate = useNavigate()
  const params = new URLSearchParams(location.search)
  const query = location.pathname === '/catalog' ? (params.get('q') ?? '') : ''
  const titles: Record<string, string> = {
    catalog: 'Каталог материалов',
    disciplines: 'Дисциплины',
    materials: 'Учебный материал',
    login: 'Вход',
    collections: 'Подборки',
    profile: 'Мой профиль',
    library: 'Моя библиотека',
    settings: 'Настройки',
  }
  const pageTitle = titles[location.pathname.split('/')[1]] ?? 'Учебная полка'
  useEffect(() => {
    document.title = `${pageTitle} — Учебная полка`
  }, [pageTitle])
  const onQueryChange = (value: string) => {
    const next =
      location.pathname === '/catalog'
        ? new URLSearchParams(location.search)
        : new URLSearchParams()
    if (location.pathname !== '/catalog') {
      next.set('all', '1')
    }
    if (value) {
      next.set('q', value)
    } else {
      next.delete('q')
    }
    next.delete('page')
    navigate(
      { pathname: '/catalog', search: next.toString() },
      { replace: location.pathname === '/catalog', flushSync: true },
    )
  }
  return (
    <>
      <Header
        query={query}
        onQueryChange={onQueryChange}
      />
      <Outlet />
    </>
  )
}
