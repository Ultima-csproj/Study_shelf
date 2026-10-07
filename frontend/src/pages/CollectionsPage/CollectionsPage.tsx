import { useMemo, useRef, type MouseEvent, type ReactNode } from "react";
import { useSearchParams } from "react-router-dom";
import { InfoIcon } from "@phosphor-icons/react/dist/csr/Info";
import { LockKeyIcon } from "@phosphor-icons/react/dist/csr/LockKey";
import { XIcon } from "@phosphor-icons/react/dist/csr/X";
import { SlidersHorizontalIcon } from "@phosphor-icons/react/dist/csr/SlidersHorizontal";
import { CollectionFilters } from "../../components/collections/CollectionFilters/CollectionFilters";
import { CollectionListItem } from "../../components/collections/CollectionListItem/CollectionListItem";
import { Pagination } from "../../components/catalog/Pagination/Pagination";
import { SearchInput } from "../../components/common/SearchInput/SearchInput";
import { Select } from "../../components/common/Select/Select";
import { useCollections } from "../../hooks/useCollections";
import { useViewer } from "../../hooks/useViewer";
import type { CollectionRequest, CollectionSort } from "../../types/collection";
import {
  readCollectionParams,
  writeCollectionParams,
} from "../../utils/collectionSearchParams";
import styles from "./CollectionsPage.module.css";

const sortOptions = [
  { value: "newest", label: "Сначала новые" },
  { value: "oldest", label: "Сначала старые" },
  { value: "title", label: "По названию" },
];

export function CollectionsPage() {
  const [params, setParams] = useSearchParams();
  const request = useMemo(() => readCollectionParams(params), [params]);
  const { result, loading, error, retry } = useCollections(request);
  const { viewer } = useViewer();
  const dialog = useRef<HTMLDialogElement>(null);
  const disciplines = result?.disciplines ?? [];
  const courses = result?.courses ?? [];
  const items = result?.items ?? [];
  const guest = viewer.kind === "guest";
  const resultCount = result?.total ?? 0;

  const change = (next: CollectionRequest) => {
    setParams(writeCollectionParams({ ...next, page: 1 }), { replace: true, flushSync: true });
  };

  const reset = () => {
    const resetRequest: CollectionRequest = {
      ...request,
      query: "",
      disciplineIds: [],
      courses: [],
    };

    change(resetRequest);
  };

  const removeDiscipline = (id: number) => {
    const disciplineIds = request.disciplineIds.filter((value) => value !== id);
    const nextRequest: CollectionRequest = {
      ...request,
      disciplineIds,
    };

    change(nextRequest);
  };

  const removeCourse = (course: number) => {
    const courses = request.courses.filter((value) => value !== course);
    const nextRequest: CollectionRequest = {
      ...request,
      courses,
    };

    change(nextRequest);
  };

  const handleQueryChange = (query: string) => {
    const nextRequest: CollectionRequest = { ...request, query };
    change(nextRequest);
  };

  const handleSortChange = (sort: string) => {
    const selectedSort = sort as CollectionSort;
    const nextRequest: CollectionRequest = { ...request, sort: selectedSort };
    change(nextRequest);
  };

  const handleOpenFilters = () => {
    dialog.current?.showModal();
  };

  const handleCloseFilters = () => {
    dialog.current?.close();
  };

  const handleBackdropClick = (event: MouseEvent<HTMLDialogElement>) => {
    const clickedBackdrop = event.target === event.currentTarget;

    if (clickedBackdrop) {
      handleCloseFilters();
    }
  };

  const filterPanel = (
    <CollectionFilters
      request={request}
      disciplines={disciplines}
      courses={courses}
      onChange={change}
      onReset={reset}
    />
  );

  let resultsContent: ReactNode;

  if (loading) {
    resultsContent = (
      <p role="status" className={styles.state}>
        Загружаем подборки…
      </p>
    );
  } else if (error) {
    resultsContent = (
      <div className={styles.state}>
        <p role="alert">{error}</p>
        <button type="button" onClick={retry}>
          Повторить загрузку
        </button>
      </div>
    );
  } else if (items.length === 0) {
    resultsContent = (
      <div className={styles.state}>
        <h2>Подборки не найдены</h2>
        <p>Попробуйте другой запрос или сбросьте фильтры.</p>
        <button type="button" onClick={reset}>
          Сбросить фильтры
        </button>
      </div>
    );
  } else {
    resultsContent = (
      <ul className={styles.list} aria-label="Публичные подборки">
        {items.map((collection) => (
          <CollectionListItem key={collection.id} collection={collection} />
        ))}
      </ul>
    );
  }

  let guestNotice: ReactNode = null;

  if (guest) {
    guestNotice = (
      <p>
        <LockKeyIcon size={23} aria-hidden="true" />
        Чтобы сохранять подборки, войдите в аккаунт.
      </p>
    );
  }

  return (
    <>
      <div className={styles.layout}>
        <aside className={styles.sidebar}>{filterPanel}</aside>
        <main id="main-content" className={styles.content} tabIndex={-1}>
          <div className={styles.heading}>
            <h1>Подборки</h1>
            <p>
              Учебные маршруты для повторения, практики и подготовки к
              экзаменам.
            </p>
          </div>
          <SearchInput
            value={request.query}
            onChange={handleQueryChange}
            label="Поиск подборок"
            placeholder="Поиск по названию или теме"
          />
          <div className={styles.toolbar}>
            <button
              type="button"
              className={styles.filterButton}
              onClick={handleOpenFilters}
            >
              <SlidersHorizontalIcon size={19} aria-hidden="true" />
              Фильтры
            </button>
            <div className={styles.chips}>
              {request.disciplineIds.map((id) => {
                const name =
                  disciplines.find((discipline) => discipline.id === id)
                    ?.name ?? `Дисциплина ${id}`;

                return (
                  <button
                    key={id}
                    type="button"
                    onClick={() => removeDiscipline(id)}
                    aria-label={`Убрать фильтр: ${name}`}
                  >
                    {name}
                    <XIcon size={16} aria-hidden="true" />
                  </button>
                );
              })}
              {request.courses.map((course) => (
                <button
                  key={course}
                  type="button"
                  onClick={() => removeCourse(course)}
                  aria-label={`Убрать фильтр: ${course} курс`}
                >
                  {course} курс
                  <XIcon size={16} aria-hidden="true" />
                </button>
              ))}
            </div>
            <Select
              value={request.sort}
              label="Сортировка подборок"
              options={sortOptions}
              onChange={handleSortChange}
            />
          </div>
          {resultsContent}
          {result && !loading && !error && (
            <Pagination
              page={result.page}
              pageCount={result.pageCount}
              total={result.total}
              pageSize={request.pageSize}
              itemLabel="подборок"
              onChange={(page) => setParams(writeCollectionParams({ ...request, page }))}
            />
          )}
          <footer className={styles.footer}>
            <p>
              <InfoIcon size={23} aria-hidden="true" />
              Публичные подборки доступны без входа.
            </p>
            {guestNotice}
          </footer>
        </main>
      </div>
      <dialog
        ref={dialog}
        className={styles.dialog}
        aria-label="Фильтры подборок"
        onClick={handleBackdropClick}
      >
        <button
          type="button"
          className={styles.close}
          aria-label="Закрыть фильтры"
          onClick={handleCloseFilters}
        >
          <XIcon size={23} aria-hidden="true" />
        </button>
        {filterPanel}
        <button
          type="button"
          className={styles.results}
          onClick={handleCloseFilters}
        >
          Показать подборки ({resultCount})
        </button>
      </dialog>
    </>
  );
}
