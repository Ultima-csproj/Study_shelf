import { Link } from "react-router-dom";
import styles from "./InformationPage.module.css";

export function InformationPage() {
  return (
    <main id="main-content" className={styles.page}>
      <h1>Страница не найдена</h1>
      <p>Проверьте адрес или вернитесь в каталог учебных материалов.</p>
      <div className={styles.links}>
        <Link to="/catalog">Открыть каталог →</Link>
      </div>
    </main>
  );
}
