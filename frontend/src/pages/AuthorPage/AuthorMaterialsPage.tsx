import { useNavigationContext } from "../../hooks/useNavigationContext";
import { useCallback, useState, type FormEvent } from "react";
import { Link, useSearchParams } from "react-router-dom";
import { PlusIcon } from "@phosphor-icons/react/dist/csr/Plus";
import { FileTextIcon } from "@phosphor-icons/react/dist/csr/FileText";
import { MagnifyingGlassIcon } from "@phosphor-icons/react/dist/csr/MagnifyingGlass";
import {
  getAuthorMaterials,
  type AuthorQuery,
} from "../../api/materialWorkspace";
import { useApiResource } from "../../hooks/useApiResource";
import { useViewer } from "../../hooks/useViewer";
import { AuthorStats } from "../../components/materials/author-list/AuthorStats";
import { AuthorMaterialCard } from "../../components/materials/author-list/AuthorMaterialCard";
import { Pagination } from "../../components/common/Pagination/Pagination";
import styles from "./AuthorPublications.module.css";

const tabs = [
  { value: "active", label: "Все" },
  { value: "draft", label: "Черновики" },
  { value: "pending_moderation", label: "На проверке" },
  { value: "published", label: "Опубликованные" },
  { value: "archived", label: "Архив" },
];

function AuthorSearch({
  query,
  order,
  onSearch,
  onOrder,
}: {
  query: string;
  order: AuthorQuery["order"];
  onSearch: (query: string) => void;
  onOrder: (order: string) => void;
}) {
  const [draft, setDraft] = useState(query);
  const submit = (event: FormEvent) => {
    event.preventDefault();
    onSearch(draft.trim());
  };
  return (
    <form className={styles.search} onSubmit={submit}>
      <div className={styles.searchField}>
        <button type="submit" aria-label="Найти материал">
          <MagnifyingGlassIcon size={22} aria-hidden="true" />
        </button>
        <input
          type="search"
          aria-label="Поиск по названию"
          maxLength={100}
          value={draft}
          placeholder="Поиск по названию…"
          onChange={(event) => setDraft(event.target.value)}
        />
      </div>
      <label>
        <span className="visually-hidden">Сортировка</span>
        <select
          aria-label="Сортировка"
          value={order}
          onChange={(event) => onOrder(event.target.value)}
        >
          <option value="newest">Сначала новые</option>
          <option value="oldest">Сначала старые</option>
          <option value="title">По названию</option>
        </select>
      </label>
    </form>
  );
}

export function AuthorMaterialsPage() {
  const { back, linkTo } = useNavigationContext("/profile");
  const { viewer } = useViewer();
  const [params, setParams] = useSearchParams();
  const requestedPage = Number(params.get("page") ?? "1");
  const page =
    Number.isSafeInteger(requestedPage) && requestedPage > 0
      ? requestedPage
      : 1;
  const query = params.get("q") ?? "";
  const requestedState = params.get("state") ?? "active";
  const state = tabs.some((tab) => tab.value === requestedState)
    ? requestedState
    : "active";
  const requestedOrder = params.get("order") ?? "newest";
  const order: AuthorQuery["order"] =
    requestedOrder === "oldest" || requestedOrder === "title"
      ? requestedOrder
      : "newest";
  const load = useCallback(
    (signal: AbortSignal) =>
      getAuthorMaterials({ page, q: query, state, order }, signal),
    [page, query, state, order],
  );
  const resource = useApiResource(load, "Не удалось загрузить ваши материалы");
  const change = (key: string, value: string) => {
    const next = new URLSearchParams(params);
    next.set(key, value);
    if (key !== "page") {
      next.delete("page");
    }
    setParams(next);
  };
  const reset = () => {
    const next = new URLSearchParams(params);
    for (const key of ["q", "state", "order", "page"]) {
      next.delete(key);
    }
    setParams(next);
  };
  const filtered = query !== "" || state !== "active" || page > 1;
  const userId = viewer.kind === "authenticated" ? viewer.user.id : 0;
  return (
    <div className={styles.page}>
      <Link className={styles.back} to={back.path}>
        ← {back.label}
      </Link>
      <header className={styles.header}>
        <div>
          <h1>Мои материалы</h1>
          <p>
            <span className={styles.desktopIntro}>
              Создавайте материалы, управляйте черновиками и следите за
              публикацией.
            </span>
            <span className={styles.mobileIntro}>
              Управляйте своими материалами
            </span>
          </p>
        </div>
        <Link className={styles.primary} to={linkTo("/author/materials/new")}>
          <PlusIcon size={20} aria-hidden="true" />
          Создать материал
        </Link>
      </header>
      <AuthorStats userId={userId} />
      <div className={styles.layout}>
        <div>
          <h2 className={styles.sectionTitle}>Материалы</h2>
          <div className={styles.filters}>
            <nav className={styles.tabs} aria-label="Состояние публикаций">
              {tabs.map((tab) => {
                const next = new URLSearchParams(params);
                next.set("state", tab.value);
                next.delete("page");
                return (
                  <Link
                    key={tab.value}
                    to={`?${next}`}
                    aria-current={state === tab.value ? "page" : undefined}
                  >
                    {tab.label}
                  </Link>
                );
              })}
            </nav>
            <AuthorSearch
              key={query}
              query={query}
              order={order}
              onSearch={(value) => change("q", value)}
              onOrder={(value) => change("order", value)}
            />
          </div>
          {resource.loading && (
            <p className={styles.notice} role="status">
              Загружаем материалы…
            </p>
          )}
          {resource.error && (
            <p className={styles.notice} role="alert">
              {resource.error}{" "}
              <button type="button" onClick={resource.retry}>
                Повторить
              </button>
            </p>
          )}
          {resource.result && resource.result.items.length === 0 && (
            <section className={styles.empty}>
              <FileTextIcon size={56} weight="light" aria-hidden="true" />
              <h2>
                {filtered
                  ? "Здесь пока нет материалов"
                  : "У вас пока нет материалов"}
              </h2>
              <p>
                {filtered
                  ? "Попробуйте другое название или вернитесь ко всем материалам."
                  : "Создайте первый материал. Черновик можно сохранить и продолжить позже."}
              </p>
              {filtered ? (
                <button type="button" onClick={reset}>
                  Сбросить фильтры
                </button>
              ) : (
                <Link
                  className={styles.primary}
                  to={linkTo("/author/materials/new")}
                >
                  <PlusIcon size={20} aria-hidden="true" />
                  Создать первый материал
                </Link>
              )}
            </section>
          )}
          {resource.result && resource.result.items.length > 0 && (
            <>
              <div className={styles.cards}>
                {resource.result.items.map((material) => (
                  <AuthorMaterialCard
                    key={material.id}
                    material={material}
                    userId={userId}
                  />
                ))}
              </div>
              <p className={styles.resultCount}>
                Показано {resource.result.items.length} из{" "}
                {resource.result.total} материалов
              </p>
              <Pagination
                page={page}
                pageCount={resource.result.pageCount}
                onChange={(value) => change("page", String(value))}
                loading={resource.loading}
              />
            </>
          )}
        </div>
      </div>
    </div>
  );
}
