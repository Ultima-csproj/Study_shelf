import { useMemo, useRef } from "react";
import { useSearchParams } from "react-router-dom";
import { XIcon } from "@phosphor-icons/react/dist/csr/X";
import { PageContainer } from "../../components/layout/PageContainer/PageContainer";
import { CatalogFilters } from "../../components/catalog/CatalogFilters/CatalogFilters";
import { CatalogSearch } from "../../components/catalog/CatalogSearch/CatalogSearch";
import { CatalogToolbar } from "../../components/catalog/CatalogToolbar/CatalogToolbar";
import { MaterialList } from "../../components/catalog/MaterialList/MaterialList";
import { Pagination } from "../../components/catalog/Pagination/Pagination";
import { Button } from "../../components/common/Button/Button";
import { emptyFilters, useCatalog } from "../../hooks/useCatalog";
import type { CatalogFilters as Filters } from "../../types/material";
import {
  readCatalogParams,
  writeCatalogParams,
} from "../../utils/catalogSearchParams";
import styles from "./CatalogPage.module.css";
import { foundMaterials } from "../../utils/format";

const PAGE_SIZE = 5;
export function CatalogPage() {
  const [params, setParams] = useSearchParams();
  const request = useMemo(() => readCatalogParams(params), [params]);
  const { filters, sort } = request;
  const dialog = useRef<HTMLDialogElement>(null);
  const { result, loading, error, retry } = useCatalog(request);
  const changeFilters = (next: Filters) => {
    setParams(writeCatalogParams(next, sort), {
      replace: true,
      flushSync: true,
    });
  };
  const reset = () => changeFilters({ ...emptyFilters });
  const changeQuery = (query: string) => changeFilters({ ...filters, query });
  const changeTag = (tag: string) =>
    changeFilters({
      ...filters,
      tags: filters.tags.includes(tag)
        ? filters.tags.filter((item) => item !== tag)
        : [...filters.tags, tag],
    });
  const changeSort = (next: typeof sort) => {
    setParams(writeCatalogParams(filters, next), {
      replace: true,
      flushSync: true,
    });
  };
  const changePage = (next: number) => {
    setParams(writeCatalogParams(filters, sort, next), { flushSync: true });
  };
  let resultLabel = foundMaterials(result.total);

  if (error) {
    resultLabel = "Материалы не загружены";
  } else if (loading) {
    resultLabel = "Загрузка материалов…";
  }
  const filterPanel = (
    <CatalogFilters
      value={filters}
      facets={result.facets}
      onChange={changeFilters}
      onReset={reset}
    />
  );
  return (
    <>
      <PageContainer>
        <div className={styles.layout}>
          <aside className={styles.sidebar}>{filterPanel}</aside>
          <main id="main-content" className={styles.content} tabIndex={-1}>
            <div className={styles.heading}>
              <h1>Каталог материалов</h1>
              <p aria-live="polite">{resultLabel}</p>
            </div>
            <CatalogSearch value={filters.query} onChange={changeQuery} />
            <CatalogToolbar
              filters={filters}
              facets={result.facets}
              sort={sort}
              onFiltersChange={changeFilters}
              onReset={reset}
              onSortChange={changeSort}
              onOpenFilters={() => dialog.current?.showModal()}
            />
            <MaterialList
              materials={result.items}
              loading={loading}
              error={error}
              selectedTags={filters.tags}
              onTagClick={changeTag}
              onReset={reset}
              onRetry={retry}
            />
            <Pagination
              page={result.page}
              pageCount={result.pageCount}
              total={result.total}
              pageSize={PAGE_SIZE}
              onChange={changePage}
            />
          </main>
        </div>
      </PageContainer>
      <dialog
        ref={dialog}
        className={styles.filterDialog}
        aria-label="Фильтры каталога"
        onClick={(event) => {
          if (event.target === event.currentTarget) {
            dialog.current?.close();
          }
        }}
      >
        <div className={styles.dialogHead}>
          <span>Фильтры каталога</span>
          <button
            type="button"
            onClick={() => dialog.current?.close()}
            aria-label="Закрыть фильтры"
          >
            <XIcon size={20} aria-hidden="true" />
          </button>
        </div>
        {filterPanel}
        <Button
          variant="secondary"
          className={styles.showResults}
          onClick={() => dialog.current?.close()}
        >
          Показать материалы ({result.total})
        </Button>
      </dialog>
    </>
  );
}
