import { createBrowserRouter, Navigate, redirect } from 'react-router-dom'
import { AppShell } from './AppShell'
import { CatalogPage } from '../pages/CatalogPage/CatalogPage'
import { DisciplinePage } from '../pages/DisciplinePage/DisciplinePage'
import { LoginPage } from '../pages/LoginPage/LoginPage'
import { InformationPage } from '../pages/InformationPage/InformationPage'
import { CollectionsPage } from '../pages/CollectionsPage/CollectionsPage'
import { CollectionPage } from '../pages/CollectionPage/CollectionPage'
import { PersonalLayout } from '../components/layout/PersonalLayout/PersonalLayout'
import { ProfilePage } from '../pages/ProfilePage/ProfilePage'
import { LibraryPage } from '../pages/LibraryPage/LibraryPage'
import { SettingsPage } from '../pages/SettingsPage/SettingsPage'
import informationStyles from '../pages/InformationPage/InformationPage.module.css'

export const router = createBrowserRouter([
  {
    element: <AppShell />,
    hydrateFallbackElement: (
      <main
        id="main-content"
        className={informationStyles.page}
      >
        <p role="status">Загружаем страницу…</p>
      </main>
    ),
    children: [
      {
        path: '/',
        element: (
          <Navigate
            to="/catalog"
            replace
          />
        ),
      },
      { path: '/catalog', element: <CatalogPage /> },
      {
        path: '/references/catalog-d',
        loader: ({ request }) => {
          const url = new URL(request.url)
          return redirect(`/catalog${url.search}${url.hash}`)
        },
      },
      {
        path: '/disciplines',
        element: (
          <Navigate
            to="/disciplines/database"
            replace
          />
        ),
      },
      { path: '/disciplines/:disciplineId', element: <DisciplinePage /> },
      {
        path: '/materials/:materialId',
        lazy: async () => {
          const { MaterialPage } =
            await import('../pages/MaterialPage/MaterialPage')
          return { Component: MaterialPage }
        },
      },
      { path: '/login', element: <LoginPage /> },
      { path: '/collections', element: <CollectionsPage /> },
      { path: '/collections/:collectionId', element: <CollectionPage /> },
      {
        element: <PersonalLayout />,
        children: [
          { path: '/profile', element: <ProfilePage /> },
          { path: '/library', element: <LibraryPage /> },
          { path: '/settings', element: <SettingsPage /> },
        ],
      },
      { path: '*', element: <InformationPage /> },
    ],
  },
])
