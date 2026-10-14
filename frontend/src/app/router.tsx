import { createBrowserRouter, redirect } from "react-router-dom";
import { HomePage } from "../pages/HomePage/HomePage";
import { AppShell } from "./AppShell";
import { RouteErrorPage } from "../pages/RouteErrorPage/RouteErrorPage";
import { CatalogPage } from "../pages/CatalogPage/CatalogPage";
import { DisciplinePage } from "../pages/DisciplinePage/DisciplinePage";
import { DisciplinesPage } from "../pages/DisciplinesPage/DisciplinesPage";
import { LoginPage } from "../pages/LoginPage/LoginPage";
import { InformationPage } from "../pages/InformationPage/InformationPage";
import { CollectionsPage } from "../pages/CollectionsPage/CollectionsPage";
import { CollectionPage } from "../pages/CollectionPage/CollectionPage";
import { PersonalLayout } from "../components/layout/PersonalLayout/PersonalLayout";
import { ProfilePage } from "../pages/ProfilePage/ProfilePage";
import { LibraryPage } from "../pages/LibraryPage/LibraryPage";
import { PersonalCollectionsPage } from "../pages/PersonalCollectionsPage/PersonalCollectionsPage";
import { PersonalCollectionPage } from "../pages/PersonalCollectionPage/PersonalCollectionPage";
import { SettingsPage } from "../pages/SettingsPage/SettingsPage";
import informationStyles from "../pages/InformationPage/InformationPage.module.css";

export const router = createBrowserRouter([
  {
    element: <AppShell />,
    errorElement: <RouteErrorPage />,
    hydrateFallbackElement: (
      <main id="main-content" className={informationStyles.page}>
        <p role="status">Загружаем страницу…</p>
      </main>
    ),
    children: [
      {
        path: "/",
        element: <HomePage />,
      },
      { path: "/catalog", element: <CatalogPage /> },
      {
        path: "/references/catalog-d",
        loader: ({ request }) => {
          const url = new URL(request.url);
          return redirect(`/catalog${url.search}${url.hash}`);
        },
      },
      {
        path: "/disciplines",
        element: <DisciplinesPage />,
      },
      { path: "/disciplines/:disciplineId", element: <DisciplinePage /> },
      {
        path: "/materials/:materialId",
        lazy: async () => {
          const { MaterialPage } =
            await import("../pages/MaterialPage/MaterialPage");
          return { Component: MaterialPage };
        },
      },
      { path: "/login", element: <LoginPage /> },
      {
        path: "/materials/:materialId/history",
        lazy: async () => ({
          Component: (
            await import("../pages/MaterialHistoryPage/MaterialHistoryPage")
          ).MaterialHistoryPage,
        }),
      },
      {
        path: "/materials/:materialId/history/:revisionId",
        lazy: async () => ({
          Component: (
            await import("../pages/MaterialHistoryPage/PublishedRevisionPage")
          ).PublishedRevisionPage,
        }),
      },
      {
        path: "/admin",
        lazy: async () => ({
          Component: (await import("../pages/AdminPage/AdminLayout"))
            .AdminLayout,
        }),
        children: [
          {
            path: "materials/pending",
            lazy: async () => ({
              Component: (
                await import("../pages/AdminPage/AdminMaterialModerationPage")
              ).AdminMaterialModerationPage,
            }),
          },
          {
            path: "materials/restricted",
            lazy: async () => ({
              Component: (
                await import("../pages/AdminPage/AdminRestrictedMaterialsPage")
              ).AdminRestrictedMaterialsPage,
            }),
          },
          {
            path: "materials/:id/revisions/:revisionId",
            lazy: async () => ({
              Component: (
                await import("../pages/AdminPage/AdminMaterialReviewPage")
              ).AdminMaterialReviewPage,
            }),
          },
          {
            index: true,
            lazy: async () => ({
              Component: (await import("../pages/AdminPage/AdminHomePage"))
                .AdminHomePage,
            }),
          },
          {
            path: "users",
            lazy: async () => ({
              Component: (await import("../pages/AdminPage/AdminUsersPage"))
                .AdminUsersPage,
            }),
          },
          {
            path: "users/:id",
            lazy: async () => ({
              Component: (await import("../pages/AdminPage/AdminUserPage"))
                .AdminUserPage,
            }),
          },
          {
            path: "institutions",
            lazy: async () => ({
              Component: (
                await import("../pages/AdminPage/AdminInstitutionsPage")
              ).AdminInstitutionsPage,
            }),
          },
          {
            path: "institutions/:id",
            lazy: async () => ({
              Component: (
                await import("../pages/AdminPage/AdminInstitutionPage")
              ).AdminInstitutionPage,
            }),
          },
          {
            path: "requests",
            lazy: async () => ({
              Component: (await import("../pages/AdminPage/AdminRequestsPage"))
                .AdminRequestsPage,
            }),
          },
          {
            path: "audit",
            lazy: async () => ({
              Component: (await import("../pages/AdminPage/AdminAuditPage"))
                .AdminAuditPage,
            }),
          },
        ],
      },
      { path: "/collections", element: <CollectionsPage /> },
      { path: "/collections/:collectionId", element: <CollectionPage /> },
      {
        element: <PersonalLayout />,
        children: [
          {
            path: "/author/materials",
            lazy: async () => ({
              Component: (
                await import("../pages/AuthorPage/AuthorMaterialsPage")
              ).AuthorMaterialsPage,
            }),
          },
          {
            path: "/author/materials/new",
            lazy: async () => ({
              Component: (
                await import("../pages/AuthorPage/CreateMaterialPage")
              ).CreateMaterialPage,
            }),
          },
          {
            path: "/author/materials/:id",
            lazy: async () => ({
              Component: (
                await import("../pages/AuthorPage/MaterialWorkspacePage")
              ).MaterialWorkspacePage,
            }),
          },
          { path: "/profile", element: <ProfilePage /> },
          { path: "/library", element: <LibraryPage /> },
          {
            path: "/library/collections",
            element: <PersonalCollectionsPage />,
          },
          {
            path: "/library/collections/:id",
            element: <PersonalCollectionPage />,
          },
          { path: "/settings", element: <SettingsPage /> },
        ],
      },
      { path: "*", element: <InformationPage /> },
    ],
  },
]);
