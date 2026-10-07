import { useCallback, useState, type FormEvent } from "react";
import { Link } from "react-router-dom";
import {
  getPersonalEntries,
  type CollectionAction,
  type PersonalSection,
} from "../../api/personalCollections";
import { useApiResource } from "../../hooks/useApiResource";
import styles from "./AccountForm.module.css";

interface PersonalSectionEditorProps {
  revision: number;
  collectionId: number;
  section: PersonalSection;
  onEdit: (action: CollectionAction) => Promise<void>;
}
export function PersonalSectionEditor({
  collectionId,
  section,
  onEdit,
  revision,
}: PersonalSectionEditorProps) {
  const [expanded, setExpanded] = useState(false);
  return (
    <>
      <h3>
        {section.title} · {section.entryCount} материалов
      </h3>
      <button
        type="button"
        aria-expanded={expanded}
        onClick={() => setExpanded((value) => !value)}
      >
        {expanded ? "Свернуть раздел" : "Открыть раздел"}
      </button>
      {expanded && (
        <SectionContent
          key={revision}
          collectionId={collectionId}
          section={section}
          onEdit={onEdit}
        />
      )}
    </>
  );
}
function SectionContent({
  collectionId,
  section,
  onEdit,
}: Omit<PersonalSectionEditorProps, "revision">) {
  const [page, setPage] = useState(1);
  const [title, setTitle] = useState(section.title);
  const [description, setDescription] = useState(section.description);
  const [materialId, setMaterialId] = useState("");
  const [moving, setMoving] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const load = useCallback(
    (signal: AbortSignal) =>
      getPersonalEntries(collectionId, section.id, page, signal),
    [collectionId, section.id, page],
  );
  const { result, loading, error, retry } = useApiResource(
    load,
    "Не удалось загрузить раздел",
  );

  const update = (event: FormEvent) => {
    event.preventDefault();
    void onEdit({
      action: "update_section",
      id: section.id,
      title,
      description,
    });
  };
  const add = (event: FormEvent) => {
    event.preventDefault();
    const id = Number(materialId);
    if (!Number.isSafeInteger(id) || id <= 0) {
      setNotice("Укажите положительный ID материала.");
      return;
    }
    void onEdit({ action: "add_entry", section: section.id, material: id });
  };
  const move = async (entry: number, direction: number) => {
    if (moving) {
      return;
    }
    setMoving(true);
    setNotice(null);
    try {
      const first = await getPersonalEntries(collectionId, section.id, 1);
      const ids = first.items.map((item) => item.id);
      for (let next = 2; next <= first.pageCount; next += 1) {
        const page = await getPersonalEntries(collectionId, section.id, next);
        ids.push(...page.items.map((item) => item.id));
      }
      const index = ids.indexOf(entry);
      const next = index + direction;
      if (index >= 0 && next >= 0 && next < ids.length) {
        [ids[index], ids[next]] = [ids[next], ids[index]];
        await onEdit({ action: "reorder_entries", section: section.id, ids });
      }
    } catch (reason) {
      setNotice(
        reason instanceof Error
          ? reason.message
          : "Не удалось изменить порядок",
      );
    } finally {
      setMoving(false);
    }
  };
  return (
    <>
      <form className={styles.form} onSubmit={update}>
        <label>
          Название раздела
          <input
            required
            maxLength={160}
            value={title}
            onChange={(event) => setTitle(event.target.value)}
          />
        </label>
        <label>
          Описание
          <textarea
            maxLength={2000}
            value={description}
            onChange={(event) => setDescription(event.target.value)}
          />
        </label>
        <button type="submit">Изменить раздел</button>
      </form>
      <button
        type="button"
        onClick={() => {
          if (
            window.confirm(
              "Удалить раздел и все его пункты из подборки? Материалы останутся в каталоге.",
            )
          ) {
            void onEdit({ action: "delete_section", id: section.id });
          }
        }}
      >
        Удалить раздел
      </button>
      <form className={styles.form} onSubmit={add}>
        <label>
          ID материала
          <input
            type="number"
            min="1"
            required
            value={materialId}
            onChange={(event) => setMaterialId(event.target.value)}
          />
        </label>
        <button type="submit">Добавить материал</button>
      </form>
      {loading && <p role="status">Загружаем пункты…</p>}
      {error && (
        <p role="alert">
          {error}{" "}
          <button type="button" onClick={retry}>
            Повторить
          </button>
        </p>
      )}
      {notice && <p role="alert">{notice}</p>}
      <ol>
        {result?.items.map((entry, index) => (
          <li key={entry.id}>
            {entry.material ? (
              <Link to={`/materials/${entry.materialId}`}>
                {entry.material.title}
              </Link>
            ) : (
              <span>Материал недоступен</span>
            )}
            <div className={styles.actions}>
              <button
                type="button"
                disabled={moving || (page === 1 && index === 0)}
                onClick={() => void move(entry.id, -1)}
              >
                Выше
              </button>
              <button
                type="button"
                disabled={
                  moving ||
                  (page === result.pageCount &&
                    index === result.items.length - 1)
                }
                onClick={() => void move(entry.id, 1)}
              >
                Ниже
              </button>
              <button
                type="button"
                onClick={() =>
                  void onEdit({
                    action: "delete_entry",
                    section: section.id,
                    id: entry.id,
                  })
                }
              >
                Убрать из раздела
              </button>
            </div>
          </li>
        ))}
      </ol>
      {result && (
        <nav aria-label={`Страницы раздела ${section.title}`}>
          <button
            type="button"
            disabled={page <= 1}
            onClick={() => setPage((value) => value - 1)}
          >
            Назад
          </button>
          <span>
            {" "}
            Страница {page} из {Math.max(1, result.pageCount)}{" "}
          </span>
          <button
            type="button"
            disabled={page >= result.pageCount}
            onClick={() => setPage((value) => value + 1)}
          >
            Далее
          </button>
        </nav>
      )}
    </>
  );
}
