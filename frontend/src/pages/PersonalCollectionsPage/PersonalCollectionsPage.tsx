import { useCallback, useState, type FormEvent } from "react";
import { Link, useNavigate, useSearchParams } from "react-router-dom";
import {
  createCollection,
  getPersonalCollections,
} from "../../api/personalCollections";
import { useApiResource } from "../../hooks/useApiResource";
import { useViewer } from "../../hooks/useViewer";
import styles from "../LibraryPage/LibraryPage.module.css";
import formStyles from "../../components/account/AccountForm.module.css";

export function PersonalCollectionsPage() {
  const { viewer } = useViewer();
  const userId = viewer.kind === "authenticated" ? viewer.user.id : null;
  const [params, setParams] = useSearchParams();
  const navigate = useNavigate();
  const [title, setTitle] = useState("");
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const requested = Number(params.get("page") ?? "1");
  const page = Number.isSafeInteger(requested) && requested > 0 ? requested : 1;
  const load = useCallback(
    (signal: AbortSignal) => {
      void userId;
      return getPersonalCollections(page, signal);
    },
    [page, userId],
  );
  const { result, loading, error, retry } = useApiResource(
    load,
    "Не удалось загрузить подборки",
  );

  const submit = async (event: FormEvent) => {
    event.preventDefault();
    if (busy) {
      return;
    }
    setBusy(true);
    setNotice(null);
    try {
      const result = await createCollection({
        title,
        shortDescription: "",
        description: "",
        purpose: "",
        targetAudience: "",
      });
      navigate(`/library/collections/${result.collection.id}`);
    } catch (reason) {
      setNotice(
        reason instanceof Error
          ? reason.message
          : "Не удалось создать подборку",
      );
    } finally {
      setBusy(false);
    }
  };
  return (
    <div className={styles.page}>
      <Link to="/library">← Моя библиотека</Link>
      <h1>Мои подборки</h1>
      <p>Новые подборки приватные и доступны только вам.</p>
      <form className={formStyles.form} onSubmit={submit}>
        <fieldset disabled={busy}>
          <label htmlFor="collection-title">Название новой подборки</label>
          <input
            id="collection-title"
            required
            maxLength={160}
            value={title}
            onChange={(event) => setTitle(event.target.value)}
          />
          <button type="submit">Создать подборку</button>
        </fieldset>
      </form>
      {notice && <p role="alert">{notice}</p>}
      {loading && <p role="status">Загружаем…</p>}
      {error && (
        <p role="alert">
          {error}{" "}
          <button type="button" onClick={retry}>
            Повторить
          </button>
        </p>
      )}
      <ul>
        {result?.items.map((item) => (
          <li key={item.id}>
            <Link to={`/library/collections/${item.id}`}>{item.title}</Link> ·{" "}
            {item.visibility === "private" ? "Приватная" : "Публичная"}
          </li>
        ))}
      </ul>
      {result && (
        <nav aria-label="Страницы собственных подборок">
          <button
            type="button"
            disabled={page <= 1}
            onClick={() => setParams({ page: String(page - 1) })}
          >
            Назад
          </button>
          <span>
            {" "}
            Страница {page} · Всего {result.total}{" "}
          </span>
          <button
            type="button"
            disabled={page >= result.pageCount}
            onClick={() => setParams({ page: String(page + 1) })}
          >
            Далее
          </button>
        </nav>
      )}
    </div>
  );
}
