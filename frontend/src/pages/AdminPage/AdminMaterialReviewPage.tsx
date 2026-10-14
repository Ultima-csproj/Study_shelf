import { useCallback, useState } from "react";
import { Link, useNavigate, useParams } from "react-router-dom";
import { getRevision, transitionRevision } from "../../api/materialWorkspace";
import { useApiResource } from "../../hooks/useApiResource";
import { useMaterialAction } from "../../hooks/useMaterialAction";
import { RevisionContent } from "../../components/materials/revisions/RevisionContent";
import styles from "../../components/materials/shared/Panel.module.css";

export function AdminMaterialReviewPage() {
  const navigate = useNavigate();
  const params = useParams();
  const material = Number(params.id);
  const revision = Number(params.revisionId);
  const [reason, setReason] = useState("");
  const [counter, setCounter] = useState(0);
  const load = useCallback(
    (signal: AbortSignal) => {
      void counter;
      return getRevision(material, revision, signal);
    },
    [material, revision, counter],
  );
  const { result, loading, error, retry } = useApiResource(
    load,
    "Редакция недоступна",
  );
  const action = useMaterialAction(() => setCounter((value) => value + 1));
  const ready = !action.busy && reason.trim().length >= 3;
  const reject = async () => {
    if (!result) {
      return;
    }
    const success = await action.run(() =>
      transitionRevision(result.revision, "reject", reason),
    );
    if (success) {
      navigate("/admin/materials/pending");
    }
  };
  return (
    <section className={styles.page}>
      <Link to="/admin/materials/pending">← Очередь модерации</Link>
      <h1>Рассмотрение редакции</h1>
      {loading && <p role="status">Загружаем…</p>}
      {error && (
        <p role="alert">
          {error}{" "}
          <button type="button" onClick={retry}>
            Обновить
          </button>
        </p>
      )}
      {action.notice && (
        <p role="status">
          {action.notice}{" "}
          <button type="button" onClick={retry}>
            Обновить данные
          </button>
        </p>
      )}
      {result && (
        <>
          <p>
            Материал #{result.material.id} · Владелец:{" "}
            {result.material.ownerName}
          </p>
          <RevisionContent value={result} />
          {result.selfReview && (
            <p className={styles.banner}>
              Вы участвовали в подготовке этой редакции. Решение будет сохранено
              с отметкой самостоятельной проверки.
            </p>
          )}
          {result.canModerate && (
            <div className={styles.form}>
              <label>
                Причина решения
                <textarea
                  required
                  minLength={3}
                  maxLength={2000}
                  value={reason}
                  onChange={(event) => setReason(event.target.value)}
                />
              </label>
              <div className={styles.actions}>
                <button
                  type="button"
                  disabled={!ready || result.outdated}
                  onClick={() =>
                    void action.run(() =>
                      transitionRevision(result.revision, "approve", reason),
                    )
                  }
                >
                  Принять и опубликовать
                </button>
                <button
                  type="button"
                  disabled={!ready}
                  onClick={() => void reject()}
                >
                  Отклонить с причиной
                </button>
              </div>
            </div>
          )}
        </>
      )}
    </section>
  );
}
