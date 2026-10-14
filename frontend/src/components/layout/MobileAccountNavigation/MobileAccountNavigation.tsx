import { navigationOrigin } from "../../../utils/navigation";
import { Link, useLocation } from "react-router-dom";
import { BookOpenIcon } from "@phosphor-icons/react/dist/csr/BookOpen";
import { HouseIcon } from "@phosphor-icons/react/dist/csr/House";
import { BookmarkSimpleIcon } from "@phosphor-icons/react/dist/csr/BookmarkSimple";
import { UserIcon } from "@phosphor-icons/react/dist/csr/User";
import { useViewer } from "../../../hooks/useViewer";
import styles from "./MobileAccountNavigation.module.css";

const links = [
  { to: "/catalog", label: "Каталог", Icon: BookOpenIcon, key: "catalog" },
  {
    to: "/",
    label: "Главная",
    Icon: HouseIcon,
    key: "home",
  },
  {
    to: "/library",
    label: "Библиотека",
    Icon: BookmarkSimpleIcon,
    key: "library",
  },
  { to: "/profile", label: "Профиль", Icon: UserIcon, key: "profile" },
];

export function MobileAccountNavigation() {
  const { viewer } = useViewer();
  const { pathname, search } = useLocation();
  if (
    viewer.kind === "loading" ||
    viewer.kind === "error" ||
    pathname.startsWith("/admin")
  ) {
    return null;
  }
  let active = "";
  if (pathname === "/") {
    active = "home";
  } else if (
    pathname.startsWith("/catalog") ||
    pathname.startsWith("/materials") ||
    pathname.startsWith("/disciplines") ||
    pathname.startsWith("/collections")
  ) {
    active = "catalog";
  } else if (pathname.startsWith("/author/")) {
    active = "profile";
  } else if (pathname.startsWith("/library")) {
    active = "library";
  } else if (pathname === "/profile" || pathname === "/settings") {
    active = "profile";
  }

  if (pathname.startsWith("/materials/")) {
    const origin = navigationOrigin(search);
    if (origin?.kind === "library" || origin?.kind === "personalCollections") {
      active = "library";
    } else if (
      origin &&
      ["profile", "author", "workspace", "settings"].includes(origin.kind)
    ) {
      active = "profile";
    }
  }

  return (
    <nav className={styles.navigation} aria-label="Нижняя навигация">
      {links.map(({ to, label, Icon, key }) => (
        <Link
          key={key}
          to={to}
          aria-current={active === key ? "page" : undefined}
        >
          <Icon
            size={24}
            weight={active === key ? "fill" : "regular"}
            aria-hidden="true"
          />
          <span>{label}</span>
        </Link>
      ))}
    </nav>
  );
}
