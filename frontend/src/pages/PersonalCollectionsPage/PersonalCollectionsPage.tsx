import { useNavigationContext } from "../../hooks/useNavigationContext";
import { Pagination } from "../../components/common/Pagination/Pagination";
import { Button } from "../../components/common/Button/Button";
import { ProfileLayout } from "../ProfilePage/ProfileLayout";
import collectionStyles from "../../components/collections/editor/CollectionWorkspace.module.css";
import { useCallback, useState, type FormEvent } from "react";
import { Link, useNavigate, useSearchParams } from "react-router-dom";
import {
  createCollection,
  getPersonalCollections,
} from "../../api/personalCollections";
import { useApiResource } from "../../hooks/useApiResource";
import { useViewer } from "../../hooks/useViewer";
import styles from "../LibraryPage/LibraryPage.module.css";
import formStyles from "../../components/common/Form/Form.module.css";

export function PersonalCollectionsPage() {
  const { back, linkTo } = useNavigationContext("/library");
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
      navigate(linkTo(`/library/collections/${result.collection.id}`));
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
  if (viewer.kind !== "authenticated") {
    return null;
  }
  return (
    <ProfileLayout user={viewer.user}>
      <div className={collectionStyles.page}>
        <Link className={collectionStyles.back} to={back.path}>
          ← {back.label}
        </Link>
        <h1>Мои подборки</h1>
        <p>Новые подборки приватные и доступны только вам.</p>
        <form
          className={`${formStyles.form} ${collectionStyles.create}`}
          onSubmit={submit}
        >
          <fieldset disabled={busy}>
            <label htmlFor="collection-title">Название новой подборки</label>
            <input
              id="collection-title"
              required
              maxLength={160}
              value={title}
              onChange={(event) => setTitle(event.target.value)}
            />
            <Button variant="primary" type="submit" loading={busy}>
              Создать подборку
            </Button>
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
        {result?.total === 0 && (
          <p className={collectionStyles.empty}>
            Создайте первую подборку и объедините материалы по теме.
          </p>
        )}
        <ul className={styles.materials}>
          {result?.items.map((item) => (
            <li key={item.id}>
              <div className={styles.itemBody}>
                <h3>
                  <Link to={linkTo(`/library/collections/${item.id}`)}>
                    {item.title}
                  </Link>
                </h3>
                <p>
                  {item.visibility === "private"
                    ? "Приватная · Только для вас"
                    : "Публичная подборка"}
                </p>
              </div>
            </li>
          ))}
        </ul>
        {result && (
          <Pagination
            page={page}
            pageCount={result.pageCount}
            loading={loading}
            onChange={(nextPage) => {
              const next = new URLSearchParams(params);
              next.set("page", String(nextPage));
              setParams(next);
            }}
          />
        )}
      </div>
    </ProfileLayout>
  );
}
