import { Link } from "react-router-dom";
import { UserIcon } from "@phosphor-icons/react/dist/csr/User";
import { EnvelopeSimpleIcon } from "@phosphor-icons/react/dist/csr/EnvelopeSimple";
import { BuildingsIcon } from "@phosphor-icons/react/dist/csr/Buildings";
import { GraduationCapIcon } from "@phosphor-icons/react/dist/csr/GraduationCap";
import { UsersIcon } from "@phosphor-icons/react/dist/csr/Users";
import { GlobeIcon } from "@phosphor-icons/react/dist/csr/Globe";
import { LockSimpleIcon } from "@phosphor-icons/react/dist/csr/LockSimple";
import { DesktopIcon } from "@phosphor-icons/react/dist/csr/Desktop";
import { PencilSimpleIcon } from "@phosphor-icons/react/dist/csr/PencilSimple";
import { ArrowRightIcon } from "@phosphor-icons/react/dist/csr/ArrowRight";
import { CaretRightIcon } from "@phosphor-icons/react/dist/csr/CaretRight";
import type { User } from "../../../types/viewer";
import { Button } from "../../common/Button/Button";
import { StatusBadge } from "../../common/StatusBadge/StatusBadge";
import { ProfileMaterials } from "./ProfileMaterials";
import styles from "./ProfileOverview.module.css";

export function ProfileOverview({
  user,
  onEdit,
}: {
  user: User;
  onEdit: () => void;
}) {
  const institutions = user.memberships.filter(
    (item) => item.state === "active",
  );
  const fields = [
    { label: "Имя", value: user.name, Icon: UserIcon },
    { label: "Email", value: user.email, Icon: EnvelopeSimpleIcon },
    {
      label: "Учебное заведение",
      value:
        institutions.map((item) => item.institutionName).join(", ") ||
        "Не указано",
      Icon: BuildingsIcon,
    },
  ];
  if (user.role === "student") {
    fields.push({
      label: "Курс",
      value: user.course === null ? "Не указан" : `${user.course} курс`,
      Icon: GraduationCapIcon,
    });
    fields.push({
      label: "Группа",
      value: user.group || "Не указана",
      Icon: UsersIcon,
    });
  }
  const privacy = [
    {
      label: "Публичный профиль",
      state: "off",
      value: "Выкл",
      Icon: GlobeIcon,
    },
    {
      label: "Показывать учреждение",
      state: "active",
      value: "Вкл",
      Icon: BuildingsIcon,
    },
    {
      label: "Показывать email",
      state: "hidden",
      value: "Скрыт",
      Icon: EnvelopeSimpleIcon,
    },
  ];
  return (
    <div className={styles.overview}>
      <section className={styles.panel}>
        <header className={styles.heading}>
          <div>
            <h2>Личный кабинет</h2>
            <p>
              Основная информация о вашем аккаунте, материалах и настройках.
            </p>
          </div>
          <Button variant="primary" size="compact" onClick={onEdit}>
            <PencilSimpleIcon size={18} aria-hidden="true" />
            Редактировать профиль
          </Button>
        </header>
        <div className={styles.columns}>
          <section className={styles.inner}>
            <h3>Основная информация</h3>
            <dl className={styles.details}>
              {fields.map(({ label, value, Icon }) => (
                <div key={label}>
                  <dt>
                    <Icon size={21} aria-hidden="true" />
                    {label}
                  </dt>
                  <dd>{value}</dd>
                </div>
              ))}
            </dl>
          </section>
          <section className={styles.inner}>
            <div className={styles.heading}>
              <h3>Видимость и приватность</h3>
              <StatusBadge state="hidden" label="Скоро" />
            </div>
            {privacy.map(({ label, state, value, Icon }) => (
              <div className={styles.row} key={label}>
                <Icon size={21} aria-hidden="true" />
                <span>{label}</span>
                <StatusBadge state={state} label={value} />
              </div>
            ))}
            <p className={styles.hint}>
              Предпросмотр. Настройки приватности и просмотр публичного профиля
              будут добавлены позже.
            </p>
          </section>
        </div>
      </section>
      <ProfileMaterials />
      <section className={styles.panel}>
        <header className={styles.heading}>
          <div>
            <h2>Учреждения и заявки</h2>
            <p>Ваши учебные заведения и заявки на вступление.</p>
          </div>
          <Link
            className={styles.outline}
            to="/settings#institution-membership"
          >
            Учреждения и авторство
            <ArrowRightIcon size={18} aria-hidden="true" />
          </Link>
        </header>
        <div className={styles.list}>
          {user.memberships.length === 0 && (
            <p className={styles.hint}>
              Вы пока не состоите ни в одном учреждении. Подайте заявку в
              разделе настроек.
            </p>
          )}
          {user.memberships.map((item) => (
            <Link
              className={styles.institution}
              key={item.institutionId}
              to="/settings#institution-membership"
            >
              <BuildingsIcon size={27} aria-hidden="true" />
              <span>
                <strong>{item.institutionName}</strong>
                <small>
                  {item.state === "pending"
                    ? "Заявка на вступление"
                    : "Учебное заведение"}
                </small>
              </span>
              <StatusBadge state={item.state} />
              <CaretRightIcon size={17} aria-hidden="true" />
            </Link>
          ))}
        </div>
      </section>
      <section className={styles.panel}>
        <header className={styles.heading}>
          <div>
            <h2>Безопасность аккаунта</h2>
            <p>Управление доступом к аккаунту и настройками безопасности.</p>
          </div>
        </header>
        <div className={styles.security}>
          <Link to="/settings">
            <EnvelopeSimpleIcon size={24} aria-hidden="true" />
            <span>
              <strong>Email</strong>
              <small>{user.email}</small>
            </span>
            <CaretRightIcon size={17} aria-hidden="true" />
          </Link>
          <Link to="/settings">
            <LockSimpleIcon size={24} aria-hidden="true" />
            <span>
              <strong>Пароль</strong>
              <small>Изменить пароль</small>
            </span>
            <CaretRightIcon size={17} aria-hidden="true" />
          </Link>
          <Link to="/settings">
            <DesktopIcon size={24} aria-hidden="true" />
            <span>
              <strong>Сессии</strong>
              <small>Управление входами</small>
            </span>
            <CaretRightIcon size={17} aria-hidden="true" />
          </Link>
        </div>
        <Link className={styles.outline} to="/settings">
          Настройки аккаунта
          <ArrowRightIcon size={18} aria-hidden="true" />
        </Link>
      </section>
    </div>
  );
}
