import { Link } from "react-router-dom";
import { getUsers, stateLabels } from "../../api/admin";
import { AdminList } from "../../components/admin/AdminList";
import styles from "../../components/admin/Admin.module.css";
export function AdminUsersPage() {
  return (
    <AdminList
      title="Аккаунты"
      load={getUsers}
      states={["active", "disabled"]}
      stateLabels={stateLabels}
      columns={[
        {
          title: "Аккаунт",
          cell: (item) => (
            <Link to={`/admin/users/${item.id}`}>{item.name}</Link>
          ),
        },
        { title: "Номер", cell: (item) => `#${item.id}` },
        {
          title: "Статус",
          cell: (item) => (
            <span className={styles.badge}>{stateLabels[item.state]}</span>
          ),
        },
        { title: "Email", cell: (item) => item.email ?? "Не показан" },
        {
          title: "Действие",
          cell: (item) => <Link to={`/admin/users/${item.id}`}>Открыть</Link>,
        },
      ]}
      render={(u) => (
        <article className={styles.card}>
          <h2>
            <Link to={`/admin/users/${u.id}`}>{u.name}</Link>
          </h2>
          <p>
            #{u.id} · {stateLabels[u.state]}
          </p>
          {u.email !== null && <p>{u.email}</p>}
        </article>
      )}
    />
  );
}
