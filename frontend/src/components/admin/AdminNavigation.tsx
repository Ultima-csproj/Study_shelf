import { useRef, type ReactNode } from "react";
import { NavLink, useLocation } from "react-router-dom";
import { SquaresFourIcon } from "@phosphor-icons/react/dist/csr/SquaresFour";
import { UsersIcon } from "@phosphor-icons/react/dist/csr/Users";
import { BuildingsIcon } from "@phosphor-icons/react/dist/csr/Buildings";
import { FileTextIcon } from "@phosphor-icons/react/dist/csr/FileText";
import { ShieldCheckIcon } from "@phosphor-icons/react/dist/csr/ShieldCheck";
import { ClipboardTextIcon } from "@phosphor-icons/react/dist/csr/ClipboardText";
import { ClockCounterClockwiseIcon } from "@phosphor-icons/react/dist/csr/ClockCounterClockwise";
import type { AdminAccess } from "../../api/admin";
import styles from "./AdminLayout.module.css";

interface AdminNavigationProps {
  access: AdminAccess;
  platform: boolean;
}
interface NavigationItem {
  label: string;
  path: string;
  visible: boolean;
  icon: ReactNode;
}
export function AdminNavigation({ access, platform }: AdminNavigationProps) {
  const details = useRef<HTMLDetailsElement>(null);
  const location = useLocation();
  const users = access.permissions.some((permission) =>
    ["manage_accounts", "manage_platform_permissions"].includes(permission),
  );
  const institutions =
    access.permissions.includes("manage_institutions") ||
    access.institutions.some((item) =>
      item.permissions.includes("manage_members"),
    );
  const restrictions =
    access.permissions.some((permission) =>
      ["restrict_public_materials", "review_restricted_materials"].includes(
        permission,
      ),
    ) ||
    access.institutions.some((item) =>
      item.permissions.some((permission) =>
        ["restrict_materials", "review_restricted_materials"].includes(
          permission,
        ),
      ),
    );
  const requests = access.permissions.some((permission) =>
    ["manage_accounts", "manage_institutions"].includes(permission),
  );
  const groups = [
    {
      label: "Управление",
      items: [
        {
          label: "Обзор",
          path: "/admin",
          visible: true,
          icon: <SquaresFourIcon />,
        },
        {
          label: "Аккаунты",
          path: "/admin/users",
          visible: users,
          icon: <UsersIcon />,
        },
        {
          label: platform ? "Учреждения" : "Мои учреждения",
          path: "/admin/institutions",
          visible: institutions,
          icon: <BuildingsIcon />,
        },
      ],
    },
    {
      label: "Материалы",
      items: [
        {
          label: "На проверке",
          path: "/admin/materials/pending",
          visible: access.permissions.includes("moderate_public_materials"),
          icon: <FileTextIcon />,
        },
        {
          label: "Ограничения",
          path: "/admin/materials/restricted",
          visible: restrictions,
          icon: <ShieldCheckIcon />,
        },
      ],
    },
    {
      label: "Система",
      items: [
        {
          label: "Заявки",
          path: "/admin/requests",
          visible: requests,
          icon: <ClipboardTextIcon />,
        },
        {
          label: "Журнал действий",
          path: "/admin/audit",
          visible: users || institutions,
          icon: <ClockCounterClockwiseIcon />,
        },
      ],
    },
  ];
  const close = () => {
    if (details.current) {
      details.current.open = false;
    }
  };
  const links = (items: NavigationItem[]) =>
    items
      .filter((item) => item.visible)
      .map((item) => (
        <NavLink
          key={item.path}
          to={item.path}
          end={item.path === "/admin"}
          onClick={close}
        >
          {item.icon}
          <span>{item.label}</span>
        </NavLink>
      ));
  const selected = groups
    .flatMap((group) => group.items)
    .find(
      (item) =>
        item.visible &&
        (item.path === "/admin"
          ? location.pathname === item.path
          : location.pathname.startsWith(item.path)),
    );
  return (
    <aside className={styles.sidebar}>
      <p className={styles.sidebarTitle}>
        {platform ? "Управление платформой" : "Управление учреждениями"}
      </p>
      <details ref={details} className={styles.mobileMenu}>
        <summary>Разделы · {selected?.label ?? "Управление"}</summary>
        <nav aria-label="Разделы управления на узком экране">
          {groups.map((group) => links(group.items))}
        </nav>
      </details>
      <nav className={styles.desktopMenu} aria-label="Административные разделы">
        {groups.map((group) => {
          if (!group.items.some((item) => item.visible)) {
            return null;
          }
          return (
            <div key={group.label} className={styles.group}>
              <p>{group.label}</p>
              {links(group.items)}
            </div>
          );
        })}
      </nav>
      {!platform && (
        <p className={styles.scopeNote}>
          Доступны только разделы ваших учреждений.
        </p>
      )}
    </aside>
  );
}
