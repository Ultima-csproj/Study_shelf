import { useRef, useState, type ReactNode } from "react";
import { Link, NavLink, useLocation } from "react-router-dom";
import { BooksIcon } from "@phosphor-icons/react/dist/csr/Books";
import { ListIcon } from "@phosphor-icons/react/dist/csr/List";
import { SearchInput } from "../../common/SearchInput/SearchInput";
import { ThemeToggle } from "../../common/ThemeToggle/ThemeToggle";
import { MobileMenu } from "../MobileMenu/MobileMenu";
import { AccountMenu } from "../AccountMenu/AccountMenu";
import { useViewer } from "../../../hooks/useViewer";
import { loginPath } from "../../../utils/redirect";
import { materialNavigationSection } from "../../../utils/navigation";
import styles from "./Header.module.css";
interface HeaderProps {
  query: string;
  onQueryChange: (value: string) => void;
}
export function Header({ query, onQueryChange }: HeaderProps) {
  const { viewer } = useViewer();
  const location = useLocation();
  const menu = useRef<HTMLDialogElement>(null);
  const [menuOpen, setMenuOpen] = useState(false);
  const login = loginPath(location);
  const materialSection = materialNavigationSection(location);
  const selectedSection = materialSection ?? location.pathname.split("/")[1];
  const catalogSelected = selectedSection === "catalog";
  const disciplinesSelected = selectedSection === "disciplines";
  const collectionsSelected = selectedSection === "collections";
  let catalogClassName = "";
  let disciplinesClassName = "";
  let collectionsClassName = "";
  let catalogCurrent: "location" | undefined = undefined;
  let disciplinesCurrent: "location" | undefined = undefined;
  let collectionsCurrent: "location" | undefined = undefined;

  if (catalogSelected) {
    catalogClassName = styles.active;
    catalogCurrent = "location";
  }

  if (disciplinesSelected) {
    disciplinesClassName = styles.active;
    disciplinesCurrent = "location";
  }

  if (collectionsSelected) {
    collectionsClassName = styles.active;
    collectionsCurrent = "location";
  }

  let accountContent: ReactNode;

  if (viewer.kind === "guest") {
    accountContent = (
      <NavLink to={login} className={styles.login}>
        Войти
      </NavLink>
    );
  } else {
    accountContent = <AccountMenu />;
  }

  const handleOpenMenu = () => {
    menu.current?.showModal();
    setMenuOpen(true);
  };

  const handleMenuClose = () => {
    setMenuOpen(false);
  };

  return (
    <>
      <a href="#main-content" className="skip-link">
        Перейти к содержимому
      </a>
      <header className={styles.header} data-site-header>
        <div className={styles.inner}>
          <NavLink
            to="/catalog"
            className={styles.brand}
            aria-label="Учебная полка — каталог"
          >
            <BooksIcon size={35} weight="fill" aria-hidden="true" />
            <span>Учебная полка</span>
          </NavLink>
          <nav aria-label="Основная навигация" className={styles.nav}>
            <Link
              to="/catalog"
              className={catalogClassName}
              aria-current={catalogCurrent}
            >
              Каталог
            </Link>
            <Link
              to="/disciplines"
              className={disciplinesClassName}
              aria-current={disciplinesCurrent}
            >
              Дисциплины
            </Link>
            <Link
              to="/collections"
              className={collectionsClassName}
              aria-current={collectionsCurrent}
            >
              Подборки
            </Link>
          </nav>
          <div className={styles.search}>
            <SearchInput
              compact
              value={query}
              onChange={onQueryChange}
              label="Поиск по каталогу в шапке"
              placeholder="Поиск по материалам, дисциплинам, авторам…"
            />
          </div>
          <ThemeToggle />
          <div className={styles.account}>
            {accountContent}
          </div>
          <button
            type="button"
            className={styles.hamburger}
            aria-label="Открыть меню"
            aria-expanded={menuOpen}
            aria-haspopup="dialog"
            onClick={handleOpenMenu}
          >
            <ListIcon size={25} aria-hidden="true" />
          </button>
        </div>
      </header>
      <MobileMenu dialogRef={menu} onClose={handleMenuClose} />
    </>
  );
}
