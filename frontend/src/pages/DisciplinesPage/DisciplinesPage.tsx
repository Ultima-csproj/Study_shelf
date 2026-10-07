import { useCallback } from "react";
import { Link, useSearchParams } from "react-router-dom";
import { getDisciplines } from "../../api/disciplines";
import { useApiResource } from "../../hooks/useApiResource";
import { PageContainer } from "../../components/layout/PageContainer/PageContainer";
import { Pagination } from "../../components/catalog/Pagination/Pagination";
import { Button } from "../../components/common/Button/Button";
import styles from "./DisciplinesPage.module.css";

export function DisciplinesPage() {
  const [params, setParams] = useSearchParams();
  const page = Math.max(1, Number.parseInt(params.get("page") ?? "1", 10) || 1);
  const load = useCallback((signal: AbortSignal) => getDisciplines(signal, page, 20), [page]);
  const { result, loading, error, retry } = useApiResource(load, "Не удалось загрузить дисциплины");
  const changePage = (next: number) => {
    const nextParams = new URLSearchParams(params);
    nextParams.set("page", String(next));
    setParams(nextParams);
  };
  return (
    <PageContainer>
      <main id="main-content" className={styles.page} tabIndex={-1}>
        <h1>Дисциплины</h1>
        <p>Учебные материалы и разделы учреждений.</p>
        {loading && <p role="status">Загрузка дисциплин…</p>}
        {error && (
          <div role="alert">
            <p>{error}</p>
            <Button variant="secondary" onClick={retry}>Повторить загрузку</Button>
          </div>
        )}
        {!loading && !error && result?.items.length === 0 && <p>Дисциплины не найдены.</p>}
        {!loading && !error && (
          <ul className={styles.list}>
            {result?.items.map((discipline) => (
              <li key={discipline.id}>
                <Link to={`/disciplines/${discipline.id}`}>
                  <h2>{discipline.name}</h2>
                  <p>{discipline.institution.name}</p>
                  <p>{discipline.materialCount} материалов · {discipline.stepCount} учебных шагов</p>
                </Link>
              </li>
            ))}
          </ul>
        )}
        {result && !loading && !error && (
          <Pagination
            page={result.page}
            pageCount={result.pageCount}
            total={result.total}
            pageSize={20}
            itemLabel="дисциплин"
            onChange={changePage}
          />
        )}
      </main>
    </PageContainer>
  );
}
