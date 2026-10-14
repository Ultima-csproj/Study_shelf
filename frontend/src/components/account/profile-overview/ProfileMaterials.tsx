import { useNavigationContext } from "../../../hooks/useNavigationContext";
import { useCallback, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { FileTextIcon } from "@phosphor-icons/react/dist/csr/FileText";
import { ClockIcon } from "@phosphor-icons/react/dist/csr/Clock";
import { BookmarkSimpleIcon } from "@phosphor-icons/react/dist/csr/BookmarkSimple";
import { ArrowRightIcon } from "@phosphor-icons/react/dist/csr/ArrowRight";
import { CaretRightIcon } from "@phosphor-icons/react/dist/csr/CaretRight";
import { getAuthorMaterials } from "../../../api/materialWorkspace";
import { getSavedMaterials } from "../../../api/library";
import { useApiResource } from "../../../hooks/useApiResource";
import { useViewer } from "../../../hooks/useViewer";
import styles from "./ProfileOverview.module.css";

export function ProfileMaterials() {
  const { linkTo } = useNavigationContext("/profile");
  const { libraryVersion } = useViewer();
  const [query, setQuery] = useState("");
  const [state, setState] = useState("active");
  const navigate = useNavigate();
  const load = useCallback(
    async (signal: AbortSignal) => {
      void libraryVersion;
      const states = ["published", "draft", "pending_moderation"];
      const [materials, saved] = await Promise.all([
        Promise.all(
          states.map((filter) =>
            getAuthorMaterials(
              { page: 1, q: "", state: filter, order: "newest" },
              signal,
            ),
          ),
        ),
        getSavedMaterials(1, signal),
      ]);
      return {
        published: materials[0].total,
        draft: materials[1].total,
        pending: materials[2].total,
        saved: saved.total,
      };
    },
    [libraryVersion],
  );
  const resource = useApiResource(
    load,
    "Не удалось загрузить статистику материалов.",
  );
  const cards = [
    {
      label: "Публикации",
      value: resource.result?.published,
      state: "published",
      Icon: FileTextIcon,
      tone: "blue",
    },
    {
      label: "Черновики",
      value: resource.result?.draft,
      state: "draft",
      Icon: FileTextIcon,
      tone: "neutral",
    },
    {
      label: "На проверке",
      value: resource.result?.pending,
      state: "pending_moderation",
      Icon: ClockIcon,
      tone: "warning",
    },
    {
      label: "Сохранено",
      value: resource.result?.saved,
      state: "saved",
      Icon: BookmarkSimpleIcon,
      tone: "blue",
    },
  ];
  const target = (filter: string) =>
    linkTo(
      `/author/materials?${new URLSearchParams({ state: filter, q: query })}`,
    );
  const filters = [
    { value: "active", label: "Все" },
    { value: "published", label: "Опубликовано" },
    { value: "draft", label: "Черновики" },
    { value: "pending_moderation", label: "На проверке" },
  ];
  return (
    <section className={styles.panel}>
      <header className={styles.heading}>
        <div>
          <h2>Мои материалы</h2>
          <p>Статистика публикаций и быстрый доступ к управлению.</p>
        </div>
        <Link className={styles.primary} to={linkTo("/author/materials")}>
          Управлять материалами
          <ArrowRightIcon size={18} aria-hidden="true" />
        </Link>
      </header>
      <nav className={styles.stats} aria-label="Статистика материалов">
        {cards.map(({ label, value, state: filter, Icon, tone }) => (
          <Link
            key={filter}
            to={filter === "saved" ? "/library" : target(filter)}
          >
            <span className={`${styles.statIcon} ${styles[tone]}`}>
              <Icon size={25} aria-hidden="true" />
            </span>
            <span>
              <small>{label}</small>
              <strong>{value ?? "—"}</strong>
            </span>
            <CaretRightIcon size={17} aria-hidden="true" />
          </Link>
        ))}
      </nav>
      {resource.error && (
        <p role="alert" className={styles.hint}>
          {resource.error}{" "}
          <button type="button" onClick={resource.retry}>
            Повторить
          </button>
        </p>
      )}
      <form
        className={styles.search}
        onSubmit={(event) => {
          event.preventDefault();
          navigate(target(state));
        }}
      >
        <input
          type="search"
          aria-label="Поиск по моим материалам"
          placeholder="Поиск по моим материалам"
          value={query}
          onChange={(event) => setQuery(event.target.value)}
        />
        <div className={styles.filters}>
          {filters.map((filter) => (
            <button
              key={filter.value}
              type="button"
              aria-pressed={state === filter.value}
              onClick={() => {
                setState(filter.value);
                navigate(target(filter.value));
              }}
            >
              {filter.label}
            </button>
          ))}
        </div>
      </form>
    </section>
  );
}
