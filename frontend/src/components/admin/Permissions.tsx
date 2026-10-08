import styles from "./Admin.module.css";
interface PermissionsProps {
  legend: string;
  labels: Record<string, string>;
  value: string[];
  onChange: (values: string[]) => void;
}
export function Permissions({
  legend,
  labels,
  value,
  onChange,
}: PermissionsProps) {
  return (
    <fieldset className={styles.permissions}>
      <legend>{legend}</legend>
      {Object.entries(labels).map(([permission, label]) => {
        const change = () => {
          onChange(
            value.includes(permission)
              ? value.filter((p) => p !== permission)
              : [...value, permission],
          );
        };
        return (
          <label key={permission}>
            <input
              type="checkbox"
              checked={value.includes(permission)}
              onChange={change}
            />
            {label}
          </label>
        );
      })}
    </fieldset>
  );
}
