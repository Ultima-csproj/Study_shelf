import type { ChangeMaterialField, MaterialFormFields } from "./types";
import styles from "./ContentForm.module.css";
interface AdditionalContentFieldsProps {
  fields: Pick<MaterialFormFields, "preview" | "duration" | "fileSize">;
  onChange: ChangeMaterialField;
}
export function AdditionalContentFields({
  fields,
  onChange,
}: AdditionalContentFieldsProps) {
  const { preview, duration, fileSize } = fields;
  return (
    <details className={styles.extra}>
      <summary>Дополнительные сведения</summary>
      <div className={styles.fields}>
        <label>
          URL обложки или превью
          <input
            type="url"
            value={preview}
            placeholder="https://example.com/image.jpg"
            onChange={(event) => onChange("preview", event.target.value)}
          />
        </label>
        <div className={styles.twoColumns}>
          <label>
            Продолжительность, секунд
            <input
              type="number"
              min={0}
              max={4294967295}
              value={duration}
              onChange={(event) => onChange("duration", event.target.value)}
            />
          </label>
          <label>
            Размер существующего файла, байт
            <input
              type="number"
              min={0}
              max={Number.MAX_SAFE_INTEGER}
              value={fileSize}
              onChange={(event) => onChange("fileSize", event.target.value)}
            />
          </label>
        </div>
        <small>Загрузка файлов будет подключена отдельно.</small>
      </div>
    </details>
  );
}
