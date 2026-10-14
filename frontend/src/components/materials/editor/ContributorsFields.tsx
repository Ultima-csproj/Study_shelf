import { PlusIcon } from "@phosphor-icons/react/dist/csr/Plus";
import { TrashIcon } from "@phosphor-icons/react/dist/csr/Trash";
import { creditLabels } from "../../../api/materialWorkspace";
import type { ContributorRole } from "../../../types/material";
import type { NamedCredit } from "./types";
import { ContributorPicker } from "./ContributorPicker";
import styles from "./ContentForm.module.css";

interface ContributorsFieldsProps {
  credits: NamedCredit[];
  busy: boolean;
  onUpdate: (index: number, value: NamedCredit) => void;
  onAdd: () => void;
  onRemove: (index: number) => void;
}
export function ContributorsFields({
  credits,
  busy,
  onUpdate,
  onAdd,
  onRemove,
}: ContributorsFieldsProps) {
  return (
    <section className={styles.section}>
      <h2>Участники материала</h2>
      <p>
        Авторство указывается отдельно от приглашений и прав редактирования.
      </p>
      {credits.map((credit, index) => (
        <div key={credit.rowKey} className={styles.credit}>
          <ContributorPicker
            id={credit.contributorId}
            name={credit.displayName}
            disabled={busy}
            onSelect={(choice) =>
              onUpdate(index, {
                ...credit,
                contributorId: choice?.id ?? 0,
                displayName: choice?.displayName,
              })
            }
          />
          <label>
            Участие
            <select
              value={credit.role}
              onChange={(event) =>
                onUpdate(index, {
                  ...credit,
                  role: event.target.value as ContributorRole,
                })
              }
            >
              {Object.entries(creditLabels).map(([value, label]) => (
                <option key={value} value={value}>
                  {label}
                </option>
              ))}
            </select>
          </label>
          <button
            type="button"
            aria-label={`Убрать участника ${credit.displayName ?? index + 1}`}
            onClick={() => onRemove(index)}
          >
            <TrashIcon size={18} aria-hidden="true" /> Убрать
          </button>
        </div>
      ))}
      <button
        type="button"
        className={styles.add}
        disabled={credits.length >= 100}
        onClick={onAdd}
      >
        <PlusIcon size={18} aria-hidden="true" /> Добавить участника
      </button>
      {credits.length === 0 && (
        <p>
          При сохранении будет указан ваш основной автор из профиля. Чтобы
          выбрать другого, добавьте участника.
        </p>
      )}
    </section>
  );
}
