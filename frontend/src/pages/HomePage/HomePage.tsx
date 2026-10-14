import { useState, type FormEvent } from "react";
import { Button } from "../../components/common/Button/Button";
import { ThemeToggle } from "../../components/common/ThemeToggle/ThemeToggle";
import { useNavigationContext } from "../../hooks/useNavigationContext";
import { Link, useNavigate } from "react-router-dom";
import { BookOpenIcon } from "@phosphor-icons/react/dist/csr/BookOpen";
import { FileTextIcon } from "@phosphor-icons/react/dist/csr/FileText";
import { BookmarkSimpleIcon } from "@phosphor-icons/react/dist/csr/BookmarkSimple";
import { GraduationCapIcon } from "@phosphor-icons/react/dist/csr/GraduationCap";
import { ArrowRightIcon } from "@phosphor-icons/react/dist/csr/ArrowRight";
import { useViewer } from "../../hooks/useViewer";
import styles from "./HomePage.module.css";

const sections = [
  {
    to: "/catalog",
    title: "Каталог",
    description: "Найти учебные материалы",
    Icon: BookOpenIcon,
  },
  {
    to: "/disciplines",
    title: "Дисциплины",
    description: "Материалы по учебным направлениям",
    Icon: GraduationCapIcon,
  },
  {
    to: "/library",
    title: "Моя библиотека",
    description: "Сохранённое и личные подборки",
    Icon: BookmarkSimpleIcon,
  },
  {
    to: "/author/materials",
    title: "Мои материалы",
    description: "Создавать и редактировать материалы",
    Icon: FileTextIcon,
  },
];

export function HomePage() {
  const { linkTo } = useNavigationContext();
  const navigate = useNavigate();
  const [query, setQuery] = useState("");
  const search = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    navigate(`/catalog?${new URLSearchParams({ q: query.trim() })}`);
  };
  const { viewer } = useViewer();
  const authenticated = viewer.kind === "authenticated";
  const visibleSections = authenticated ? sections : sections.slice(0, 2);
  return (
    <main id="main-content" className={styles.page}>
      <header>
        <div className={styles.topLine}>
          <p className={styles.eyebrow}>Учебная полка</p>
          <span className={styles.mobileTheme}>
            <ThemeToggle />
          </span>
        </div>
        <h1>Главная</h1>
        <p className={styles.intro}>
          Учебные материалы и ваши разделы в одном месте.
        </p>
      </header>
      <form
        className={styles.search}
        onSubmit={search}
        role="search"
        aria-label="Поиск материалов"
      >
        <input
          type="search"
          aria-label="Что хотите изучить?"
          placeholder="Название, тема или автор…"
          maxLength={100}
          value={query}
          onChange={(event) => setQuery(event.target.value)}
        />
        <Button type="submit" variant="primary">
          Найти
        </Button>
      </form>
      {authenticated && (
        <div className={styles.authorAction}>
          <div>
            <strong>Поделитесь знаниями</strong>
            <p>Создайте материал и продолжайте работу с черновиками.</p>
          </div>
          <Link to={linkTo("/author/materials/new")}>Создать материал →</Link>
        </div>
      )}
      <nav className={styles.sections} aria-label="Быстрые переходы главной">
        {visibleSections.map(({ to, title, description, Icon }) => (
          <Link key={to} to={to.startsWith("/author/") ? linkTo(to) : to}>
            <Icon size={28} aria-hidden="true" />
            <span>
              <strong>{title}</strong>
              <small>{description}</small>
            </span>
            <ArrowRightIcon size={20} aria-hidden="true" />
          </Link>
        ))}
      </nav>
      <Link className={styles.login} to="/collections">
        Посмотреть учебные подборки →
      </Link>
      {!authenticated && (
        <Link className={styles.login} to="/login">
          Войти в личный кабинет
        </Link>
      )}
    </main>
  );
}
