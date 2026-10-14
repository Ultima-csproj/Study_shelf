import { Link } from "react-router-dom";
import { AdminList } from "../../components/admin/AdminList";
import { getModerationMaterials } from "../../api/materialWorkspace";
import styles from "../../components/materials/shared/Panel.module.css";

export function AdminMaterialModerationPage() {
  return (
    <AdminList
      title="Модерация материалов"
      load={getModerationMaterials}
      emptyMessage="Сейчас нет материалов на проверке"
      render={(material) => (
        <article className={styles.card}>
          <p>Материал на проверке</p>
          <h2>{material.title}</h2>
          <p>
            {material.ownerName} · Материал #{material.id}
          </p>
          <p>
            Автор отправил редакцию на публикацию. Проверьте содержание и
            примите решение.
          </p>
          {material.workingRevisionId !== null && (
            <Link
              to={
                "/admin/materials/" +
                material.id +
                "/revisions/" +
                material.workingRevisionId
              }
            >
              Проверить материал
            </Link>
          )}
        </article>
      )}
    />
  );
}
