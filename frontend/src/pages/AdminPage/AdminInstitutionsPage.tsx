import { Link } from "react-router-dom";
import { getInstitutions } from "../../api/admin";
import { AdminList } from "../../components/admin/AdminList";
import styles from "../../components/admin/Admin.module.css";
export function AdminInstitutionsPage() {
  return (
    <AdminList
      title="Учреждения"
      load={getInstitutions}
      columns={[
        {
          title: "Учреждение",
          cell: (item) => (
            <Link to={`/admin/institutions/${item.id}`}>{item.name}</Link>
          ),
        },
        { title: "Номер", cell: (item) => `#${item.id}` },
        { title: "Участники", cell: (item) => item.memberCount },
        { title: "Управляющие", cell: (item) => item.managerCount },
        {
          title: "Действие",
          cell: (item) => (
            <Link to={`/admin/institutions/${item.id}`}>Открыть</Link>
          ),
        },
      ]}
      render={(i) => (
        <article className={styles.card}>
          <h2>
            <Link to={`/admin/institutions/${i.id}`}>{i.name}</Link>
          </h2>
          <p>
            #{i.id} · Участников: {i.memberCount} · Управляющих:{" "}
            {i.managerCount}
          </p>
        </article>
      )}
    />
  );
}
