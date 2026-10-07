import { useCallback, useState, type FormEvent } from "react";
import { Link, useParams } from "react-router-dom";
import {
  editCollection,
  getPersonalCollection,
  type CollectionAction,
} from "../../api/personalCollections";
import { useApiResource } from "../../hooks/useApiResource";
import { useViewer } from "../../hooks/useViewer";
import { CollectionInfoForm } from "../../components/account/CollectionInfoForm";
import { PersonalSectionEditor } from "../../components/account/PersonalSectionEditor";
import styles from "../LibraryPage/LibraryPage.module.css";
import formStyles from "../../components/account/AccountForm.module.css";

export function PersonalCollectionPage() {
  const { viewer } = useViewer();
  const userId = viewer.kind === "authenticated" ? viewer.user.id : null;
  const { id: rawId } = useParams();
  const id = Number(rawId);
  const [version, setVersion] = useState(0);
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const [sectionTitle, setSectionTitle] = useState("");
  const load = useCallback(
    (signal: AbortSignal) => {
      void version;
      void userId;
      return getPersonalCollection(id, signal);
    },
    [id, version, userId],
  );
  const { result, loading, error, retry } = useApiResource(
    load,
    "Не удалось загрузить подборку",
  );

  const run = async (action: CollectionAction) => {
    if (!result || busy) {
      return;
    }
    setBusy(true);
    setNotice(null);
    try {
      await editCollection(id, result.collection.editVersion, action);
      setVersion((value) => value + 1);
    } catch (reason) {
      setNotice(
        reason instanceof Error
          ? reason.message
          : "Не удалось изменить подборку",
      );
    } finally {
      setBusy(false);
    }
  };
  const addSection = (event: FormEvent) => {
    event.preventDefault();
    void run({ action: "add_section", title: sectionTitle, description: "" });
  };
  const moveSection = (index: number, direction: number) => {
    if (!result) {
      return;
    }
    const ids = result.sections.map((section) => section.id);
    const next = index + direction;
    if (next < 0 || next >= ids.length) {
      return;
    }
    [ids[index], ids[next]] = [ids[next], ids[index]];
    void run({ action: "reorder_sections", ids });
  };
  return (
    <div className={styles.page}>
      <Link to="/library/collections">← Мои подборки</Link>
      <h1>{result?.collection.title ?? "Подборка"}</h1>
      {loading && <p role="status">Загружаем…</p>}
      {error && (
        <p role="alert">
          {error}{" "}
          <button type="button" onClick={retry}>
            Повторить
          </button>
        </p>
      )}
      {notice && (
        <p role="alert">
          {notice}{" "}
          <button type="button" onClick={retry}>
            Обновить данные
          </button>
        </p>
      )}
      {result && result.collection.visibility === "public" && (
        <p>
          Редактирование публичных подборок подключим после согласования правил
          публикации.
        </p>
      )}
      {result && result.collection.visibility === "private" && (
        <fieldset disabled={busy || loading} style={{ border: 0, padding: 0 }}>
          <CollectionInfoForm
            key={result.collection.editVersion}
            info={result.collection}
            busy={busy}
            onSave={(info) => run({ action: "info", ...info })}
          />
          <h2>Разделы</h2>
          <form className={formStyles.form} onSubmit={addSection}>
            <label htmlFor="section-title">Название нового раздела</label>
            <input
              id="section-title"
              required
              maxLength={160}
              value={sectionTitle}
              onChange={(event) => setSectionTitle(event.target.value)}
            />
            <button type="submit">Добавить раздел</button>
          </form>
          {result.sections.map((section, index) => (
            <section className={formStyles.section} key={section.id}>
              <div className={formStyles.actions}>
                <button
                  type="button"
                  disabled={index === 0}
                  onClick={() => moveSection(index, -1)}
                >
                  Раздел выше
                </button>
                <button
                  type="button"
                  disabled={index === result.sections.length - 1}
                  onClick={() => moveSection(index, 1)}
                >
                  Раздел ниже
                </button>
              </div>
              <PersonalSectionEditor
                collectionId={id}
                revision={result.collection.editVersion}
                section={section}
                onEdit={run}
              />
            </section>
          ))}
        </fieldset>
      )}
    </div>
  );
}
