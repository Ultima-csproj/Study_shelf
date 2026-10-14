import { isRouteErrorResponse, Link, useRouteError } from "react-router-dom";
import { Button } from "../../components/common/Button/Button";
import styles from "./RouteErrorPage.module.css";

export function RouteErrorPage() {
  const error = useRouteError();
  const missing = isRouteErrorResponse(error) && error.status === 404;
  return (
    <main id="main-content" className={styles.page}>
      <section>
        <p className={styles.eyebrow}>Учебная полка</p>
        <h1>
          {missing ? "Страница не найдена" : "Не удалось открыть страницу"}
        </h1>
        <p>
          {missing
            ? "Возможно, адрес изменился. Перейдите в каталог, чтобы найти материал."
            : "Попробуйте обновить страницу. Если ошибка повторится, вернитесь в каталог."}
        </p>
        <div className={styles.actions}>
          {!missing && (
            <Button variant="primary" onClick={() => window.location.reload()}>
              Обновить страницу
            </Button>
          )}
          <Link to="/catalog">Открыть каталог →</Link>
        </div>
      </section>
    </main>
  );
}
