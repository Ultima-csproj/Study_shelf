import { Link } from "react-router-dom";
import { AdminList } from "../../components/admin/AdminList";
import { getAuthorMaterials } from "../../api/materialWorkspace";
import styles from "../../components/materialWorkspace/Workspace.module.css";

export function AuthorMaterialsPage() {
  return (
    <div className={styles.page}>
      <div className={styles.header}>
        <h1>Мои публикации</h1>
        <Link to="/author/materials/new">Создать материал</Link>
      </div>
      <p>
        Ваши материалы, материалы учреждения и доступные вам рабочие редакции.
      </p>
      <AdminList
        title="Материалы"
        heading="h2"
        load={getAuthorMaterials}
        states={["active", "archived"]}
        stateLabels={{ active: "Активные", archived: "В архиве" }}
        render={(material) => (
          <article className={styles.card}>
            <h2>
              <Link to={"/author/materials/" + material.id}>
                {material.title}
              </Link>
            </h2>
            <p>{material.ownerName}</p>
            <p>
              {material.visibility === "private" ? "Приватный" : "Публичный"} ·{" "}
              {material.publication === "published"
                ? "Опубликован"
                : "Не опубликован"}
              {material.lifecycle === "archived" && " · В архиве"}
            </p>
            {material.workingRevisionId !== null && (
              <p>Есть рабочая редакция #{material.workingRevisionId}.</p>
            )}
          </article>
        )}
      />
    </div>
  );
}
