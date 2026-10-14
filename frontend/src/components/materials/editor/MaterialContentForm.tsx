import { MainContentFields } from "./MainContentFields";
import { ContributorsFields } from "./ContributorsFields";
import { AdditionalContentFields } from "./AdditionalContentFields";
import { MaterialFormActions } from "./MaterialFormActions";
import { useMaterialContentForm } from "./useMaterialContentForm";
import type { MaterialContentFormProps } from "./types";
import styles from "./ContentForm.module.css";

export function MaterialContentForm({
  initial,
  mode,
  busy,
  allowSubmit,
  ownerField,
  onSave,
}: MaterialContentFormProps) {
  const {
    fields,
    reason,
    error,
    changeField,
    setReason,
    updateCredit,
    addCredit,
    removeCredit,
    submit,
  } = useMaterialContentForm({ initial, mode, busy, onSave });
  const isNewVersion = mode === "new_revision";
  return (
    <form className={styles.form} onSubmit={submit}>
      <fieldset disabled={busy} className={styles.fields}>
        <MainContentFields fields={fields} onChange={changeField} />
        <ContributorsFields
          credits={fields.credits}
          busy={busy}
          onUpdate={updateCredit}
          onAdd={addCredit}
          onRemove={removeCredit}
        />
        {ownerField}
        <AdditionalContentFields fields={fields} onChange={changeField} />
        {isNewVersion && (
          <label>
            Что изменилось?
            <textarea
              required
              rows={3}
              minLength={3}
              maxLength={2000}
              value={reason}
              placeholder="Например, исправлены примеры в разделе 3"
              onChange={(event) => setReason(event.target.value)}
            />
            <small>
              Это описание ваших изменений. Решение и замечания модератора
              появятся отдельно.
            </small>
          </label>
        )}

        {error && <p role="alert">{error}</p>}
        <MaterialFormActions
          mode={mode}
          busy={busy}
          allowSubmit={allowSubmit}
        />
      </fieldset>
    </form>
  );
}
