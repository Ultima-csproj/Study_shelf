import { useNavigationContext } from "../../hooks/useNavigationContext";
import type { ReactNode } from "react";
import { NavLink } from "react-router-dom";
import { UserIcon } from "@phosphor-icons/react/dist/csr/User";
import { BuildingsIcon } from "@phosphor-icons/react/dist/csr/Buildings";
import { FileTextIcon } from "@phosphor-icons/react/dist/csr/FileText";
import { PlusIcon } from "@phosphor-icons/react/dist/csr/Plus";
import { BookmarkSimpleIcon } from "@phosphor-icons/react/dist/csr/BookmarkSimple";
import { BooksIcon } from "@phosphor-icons/react/dist/csr/Books";
import { CaretRightIcon } from "@phosphor-icons/react/dist/csr/CaretRight";
import type { User } from "../../types/viewer";
import { roleLabels } from "../../utils/viewerLabels";
import styles from "./ProfileIdentityCard.module.css";

const actions = [
  {
    to: "/author/materials/new",
    title: "Создать материал",
    description: "Начать с черновика",
    Icon: PlusIcon,
  },
  {
    to: "/author/materials",
    title: "Мои материалы",
    description: "Материалы и рабочие редакции",
    Icon: FileTextIcon,
  },
  {
    to: "/library",
    title: "Библиотека",
    description: "Материалы и коллекции",
    Icon: BookmarkSimpleIcon,
  },
  {
    to: "/library/collections",
    title: "Мои подборки",
    description: "Создавать и управлять",
    Icon: BooksIcon,
  },
];

interface ProfileIdentityCardProps {
  user: User;
  editAction: ReactNode;
}

export function ProfileIdentityCard({
  user,
  editAction,
}: ProfileIdentityCardProps) {
  const { linkTo } = useNavigationContext("/profile");
  const institution = user.memberships.find(
    (membership) => membership.state === "active",
  );
  const education = [];

  if (user.role === "student" && user.course !== null) {
    education.push(`${user.course} курс`);
  }
  if (user.role === "student" && user.group) {
    education.push(user.group);
  }

  return (
    <aside className={styles.card} aria-label="Карточка профиля">
      <div className={styles.identity}>
        <div className={styles.avatar} role="img" aria-label="Аватар профиля">
          <UserIcon size={70} weight="fill" aria-hidden="true" />
        </div>
        <h1>{user.name}</h1>
        <span className={styles.role}>{roleLabels[user.role]}</span>
      </div>
      <div className={styles.education}>
        <BuildingsIcon size={26} aria-hidden="true" />
        <div>
          <p>
            {institution?.institutionName ?? "Учебное заведение не указано"}
          </p>
          {education.length > 0 && <small>{education.join(" · ")}</small>}
        </div>
      </div>
      <p className={styles.biography}>
        {user.biography || "Расскажите о себе и своих учебных интересах."}
      </p>
      <div className={styles.editAction}>{editAction}</div>
      <nav className={styles.actions} aria-label="Быстрые действия">
        <h2>Быстрые действия</h2>
        {actions.map(({ to, title, description, Icon }) => (
          <NavLink
            key={to}
            to={
              to.startsWith("/author/") || to === "/library/collections"
                ? linkTo(to)
                : to
            }
          >
            <span className={styles.actionIcon}>
              <Icon size={24} aria-hidden="true" />
            </span>
            <span className={styles.actionLabel}>
              <strong>{title}</strong>
              <small>{description}</small>
            </span>
            <CaretRightIcon size={18} aria-hidden="true" />
          </NavLink>
        ))}
      </nav>
    </aside>
  );
}
