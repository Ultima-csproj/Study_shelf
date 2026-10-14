import { useEffect } from "react";
import {
  Outlet,
  ScrollRestoration,
  useLocation,
  useNavigate,
} from "react-router-dom";
import { Header } from "../components/layout/Header/Header";
import { MobileAccountNavigation } from "../components/layout/MobileAccountNavigation/MobileAccountNavigation";
export function AppShell() {
  const location = useLocation();
  const navigate = useNavigate();
  const params = new URLSearchParams(location.search);
  let query = "";

  if (location.pathname === "/catalog") {
    query = params.get("q") ?? "";
  }
  const titles: Record<string, string> = {
    catalog: "Каталог материалов",
    disciplines: "Дисциплины",
    materials: "Учебный материал",
    login: "Вход",
    collections: "Подборки",
    profile: "Мой профиль",
    library: "Моя библиотека",
    settings: "Настройки",
  };
  const isPublicationList = location.pathname === "/author/materials";
  const isPublicationWorkspace = /^\/author\/materials\/\d+$/.test(
    location.pathname,
  );
  const pageTitle =
    location.pathname === "/"
      ? "Главная"
      : isPublicationList
        ? "Мои материалы"
        : isPublicationWorkspace
          ? "Рабочая область материала"
          : (titles[location.pathname.split("/")[1]] ?? "Учебная полка");

  useEffect(() => {
    document.title = `${pageTitle} — Учебная полка`;
  }, [pageTitle]);

  const onQueryChange = (value: string) => {
    let next = new URLSearchParams();

    if (location.pathname === "/catalog") {
      next = new URLSearchParams(location.search);
    }

    if (location.pathname !== "/catalog") {
      next.set("all", "1");
    }
    if (value) {
      next.set("q", value);
    } else {
      next.delete("q");
    }
    next.delete("page");
    navigate(
      { pathname: "/catalog", search: next.toString() },
      { replace: location.pathname === "/catalog", flushSync: true },
    );
  };
  return (
    <>
      <Header query={query} onQueryChange={onQueryChange} />
      <Outlet />
      <MobileAccountNavigation />
      <ScrollRestoration />
    </>
  );
}
