import type { MaterialType } from "../../../types/material";
import { materialTypeLabels } from "../../../utils/materialLabels";
import type { ChangeMaterialField, MaterialFormFields } from "./types";
import styles from "./ContentForm.module.css";

interface MainContentFieldsProps {
  fields: Pick<
    MaterialFormFields,
    "title" | "description" | "kind" | "year" | "tags"
  >;
  onChange: ChangeMaterialField;
}
export function MainContentFields({
  fields,
  onChange,
}: MainContentFieldsProps) {
  const { title, description, kind, year, tags } = fields;
  return (
    <>
      {" "}
      <label>
        Название
        <input
          required
          maxLength={300}
          value={title}
          placeholder="Например, основы программирования"
          onChange={(event) => onChange("title", event.target.value)}
        />
      </label>
      <label>
        Тип материала
        <select
          value={kind}
          onChange={(event) =>
            onChange("kind", event.target.value as MaterialType)
          }
        >
          {Object.entries(materialTypeLabels).map(([value, label]) => (
            <option key={value} value={value}>
              {label}
            </option>
          ))}
        </select>
      </label>
      <label>
        Текст и описание
        <textarea
          rows={7}
          maxLength={20000}
          value={description}
          placeholder="Опишите содержание материала…"
          onChange={(event) => onChange("description", event.target.value)}
        />
        <small className={styles.counter}>
          {description.length.toLocaleString("ru-RU")} / 20 000
        </small>
      </label>
      <div className={styles.twoColumns}>
        <label>
          Год источника
          <input
            type="number"
            min={1}
            max={9999}
            value={year}
            placeholder="Например, 2026"
            onChange={(event) => onChange("year", event.target.value)}
          />
        </label>
        <label>
          Теги через запятую
          <input
            maxLength={8100}
            value={tags}
            placeholder="программирование, алгоритмы"
            onChange={(event) => onChange("tags", event.target.value)}
          />
        </label>
      </div>
    </>
  );
}
