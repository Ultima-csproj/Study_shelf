import { Link, useParams } from "react-router-dom";
import type { ReactNode } from "react";
import { LinkIcon } from "@phosphor-icons/react/dist/csr/Link";
import { ArrowRightIcon } from "@phosphor-icons/react/dist/csr/ArrowRight";
import { useDiscipline } from "../../hooks/useDiscipline";
import { DisciplineSidebar } from "../../components/discipline/DisciplineSidebar/DisciplineSidebar";
import { DisciplineSelector } from "../../components/discipline/DisciplineSelector/DisciplineSelector";
import { DisciplineHeader } from "../../components/discipline/DisciplineHeader/DisciplineHeader";
import { MaterialSection } from "../../components/discipline/MaterialSection/MaterialSection";
import { Button } from "../../components/common/Button/Button";
import styles from "./DisciplinePage.module.css";
export function DisciplinePage() {
  const { disciplineId = "database" } = useParams();
  const { result, loading, error, retry } = useDiscipline(disciplineId);

  if (loading) {
    return (
      <main
        id="main-content"
        className={styles.state}
        tabIndex={-1}
        role="status"
      >
        Загрузка дисциплины…
      </main>
    );
  }

  if (error) {
    return (
      <main
        id="main-content"
        className={styles.state}
        tabIndex={-1}
        role="alert"
      >
        <h1>Не удалось загрузить дисциплину</h1>
        <p>{error}</p>
        <Button variant="secondary" onClick={retry}>
          Повторить загрузку
        </Button>
      </main>
    );
  }

  const discipline = result?.discipline;

  if (!result || !discipline) {
    return (
      <main id="main-content" className={styles.state} tabIndex={-1}>
        <h1>Дисциплина не найдена</h1>
        <p>Выберите дисциплину из каталога.</p>
        <Link to="/disciplines/database">Открыть дисциплины</Link>
      </main>
    );
  }

  const { navigation, sections } = result;
  const menu = navigation.filter(
    (item, index) => index < 5 || item.id === discipline.id,
  );
  const count = sections.reduce(
    (total, section) => total + section.items.length,
    0,
  );
  let emptyState: ReactNode = null;

  if (count === 0) {
    emptyState = (
      <div className={styles.empty} role="status">
        <h2>Материалы пока не добавлены</h2>
        <p>Выберите другую дисциплину или посмотрите общий каталог.</p>
        <Link to="/catalog?all=1">Открыть каталог</Link>
      </div>
    );
  }

  return (
    <div className={styles.layout}>
      <DisciplineSidebar disciplines={menu} selectedId={discipline.id} />
      <main id="main-content" className={styles.content} tabIndex={-1}>
        <DisciplineSelector disciplines={navigation} value={discipline.slug} />
        <DisciplineHeader key={discipline.id} discipline={discipline} />
        {sections.map((section) => (
          <MaterialSection key={section.id} section={section} />
        ))}
        {emptyState}
        <footer className={styles.footer}>
          <Link to="/collections">
            <LinkIcon size={21} aria-hidden="true" />
            <span>Материалы из подборки «Программирование и базы данных»</span>
            <ArrowRightIcon size={18} aria-hidden="true" />
          </Link>
          <Link to={`/catalog?discipline=${discipline.id}`}>
            Все материалы дисциплины ({discipline.materialCount})
            <ArrowRightIcon size={18} aria-hidden="true" />
          </Link>
        </footer>
      </main>
    </div>
  );
}
