import { useEffect, useId, useState } from "react";
import {
  findMaterialContributors,
  type ContributorChoice,
} from "../../../api/materialWorkspace";
import styles from "./ContentForm.module.css";

interface ContributorPickerProps {
  id: number;
  name?: string;
  disabled: boolean;
  onSelect: (value: ContributorChoice | null) => void;
}

export function ContributorPicker({
  id,
  name,
  disabled,
  onSelect,
}: ContributorPickerProps) {
  const inputId = useId();
  const [query, setQuery] = useState(name ?? "");
  const [choices, setChoices] = useState<ContributorChoice[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [searched, setSearched] = useState(false);
  const [attempt, setAttempt] = useState(0);
  useEffect(() => {
    if (id > 0 || query.trim().length < 2) {
      return;
    }
    const controller = new AbortController();
    const timer = setTimeout(async () => {
      setLoading(true);
      setError(null);
      try {
        const result = await findMaterialContributors(query, controller.signal);
        if (!controller.signal.aborted) {
          setChoices(result);
          setSearched(true);
        }
      } catch (reason) {
        if (!controller.signal.aborted) {
          setError(
            reason instanceof Error
              ? reason.message
              : "Не удалось найти автора",
          );
        }
      } finally {
        if (!controller.signal.aborted) {
          setLoading(false);
        }
      }
    }, 250);
    return () => {
      clearTimeout(timer);
      controller.abort();
    };
  }, [query, id, attempt]);

  return (
    <div className={styles.picker}>
      <label htmlFor={inputId}>Автор или участник</label>
      <input
        id={inputId}
        required
        maxLength={100}
        value={query}
        placeholder="Начните вводить имя автора"
        disabled={disabled}
        autoComplete="off"
        onChange={(event) => {
          setQuery(event.target.value);
          setChoices([]);
          setError(null);
          setLoading(false);
          setSearched(false);
          onSelect(null);
        }}
      />
      {id === 0 && (
        <small>Введите от двух символов и выберите автора из списка.</small>
      )}
      {loading && <small role="status">Ищем авторов…</small>}
      {id === 0 && searched && !loading && !error && choices.length === 0 && (
        <small role="status">
          Автор не найден. Попробуйте другое имя. Здесь доступны авторы
          публичных материалов и ваша запись.
        </small>
      )}
      {error && (
        <p role="alert">
          {error}{" "}
          <button
            type="button"
            onClick={() => setAttempt((value) => value + 1)}
          >
            Повторить
          </button>
        </p>
      )}
      {id === 0 && choices.length > 0 && (
        <ul className={styles.choices} aria-label="Найденные авторы">
          {choices.map((choice) => (
            <li key={choice.id}>
              <button
                type="button"
                onClick={() => {
                  setQuery(choice.displayName);
                  setChoices([]);
                  onSelect(choice);
                }}
              >
                {choice.displayName} <small>#{choice.id}</small>
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
