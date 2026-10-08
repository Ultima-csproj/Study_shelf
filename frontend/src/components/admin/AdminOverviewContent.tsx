import { Link } from "react-router-dom";
import type { AdminOverview } from "../../api/admin";
import styles from "./AdminOverview.module.css";

interface AdminOverviewContentProps {
  overview: AdminOverview;
}
interface MetricProps {
  label: string;
  value: number;
  to: string;
  hint: string;
}
function Metric({ label, value, to, hint }: MetricProps) {
  return (
    <Link to={to} className={styles.metric}>
      <span>{label}</span>
      <strong>{value}</strong>
      <small>{hint}</small>
    </Link>
  );
}
export function AdminOverviewContent({ overview }: AdminOverviewContentProps) {
  const requests =
    overview.pendingMembers +
    overview.pendingContributors +
    overview.pendingInstitutions;
  const attention = requests + (overview.pendingMaterials ?? 0);
  return (
    <>
      <div className={styles.metrics}>
        {overview.accounts !== null && (
          <Metric
            label="Аккаунты"
            value={overview.accounts}
            to="/admin/users"
            hint="Открыть список"
          />
        )}
        {overview.institutions !== null && (
          <Metric
            label="Доступные учреждения"
            value={overview.institutions}
            to="/admin/institutions"
            hint="Перейти к управлению"
          />
        )}
        <Metric
          label="Ожидающие заявки"
          value={requests}
          to="#admin-attention"
          hint="Посмотреть, что требует решения"
        />
        {overview.pendingMaterials !== null && (
          <Metric
            label="Материалы на проверке"
            value={overview.pendingMaterials}
            to="/admin/materials/pending"
            hint="Открыть очередь"
          />
        )}
      </div>
      <section id="admin-attention" className={styles.attention}>
        <h2>Требует внимания</h2>
        {attention === 0 && (
          <p className={styles.quiet}>
            В доступных вам разделах нет ожидающих решений.
          </p>
        )}
        {overview.memberAttention.map((item) => (
          <Link
            key={item.institutionId}
            className={styles.task}
            to={`/admin/institutions/${item.institutionId}?section=requests&state=pending`}
          >
            <div>
              <strong>Заявки на вступление</strong>
              <p>{item.institutionName}</p>
              <span>Посмотреть заявки</span>
            </div>
            <span className={styles.count}>{item.pendingCount}</span>
          </Link>
        ))}
        {overview.pendingContributors > 0 && (
          <Link
            className={styles.task}
            to="/admin/requests?kind=contributors&state=pending"
          >
            <div>
              <strong>Привязка авторов</strong>
              <p>Пользователи подтвердили сведения об авторстве.</p>
              <span>Рассмотреть заявки</span>
            </div>
            <span className={styles.count}>{overview.pendingContributors}</span>
          </Link>
        )}
        {overview.pendingInstitutions > 0 && (
          <Link
            className={styles.task}
            to="/admin/requests?kind=institutions&state=pending"
          >
            <div>
              <strong>Регистрация учреждений</strong>
              <p>Новые учреждения ожидают подтверждения.</p>
              <span>Рассмотреть заявки</span>
            </div>
            <span className={styles.count}>{overview.pendingInstitutions}</span>
          </Link>
        )}
        {(overview.pendingMaterials ?? 0) > 0 && (
          <Link className={styles.task} to="/admin/materials/pending">
            <div>
              <strong>Материалы на проверке</strong>
              <p>Авторы отправили новые редакции на публикацию.</p>
              <span>Проверить материалы</span>
            </div>
            <span className={styles.count}>{overview.pendingMaterials}</span>
          </Link>
        )}
        {overview.pendingMembers >
          overview.memberAttention.reduce(
            (sum, item) => sum + item.pendingCount,
            0,
          ) && (
          <Link to="/admin/institutions">Другие учреждения с заявками</Link>
        )}
      </section>
    </>
  );
}
