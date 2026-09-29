import { useEffect, useState } from "react";

export function useApiResource<T>(
  load: (signal: AbortSignal) => Promise<T>,
  errorMessage: string,
) {
  const [attempt, setAttempt] = useState(0);
  const [state, setState] = useState<{
    load: typeof load | null;
    attempt: number;
    result: T | null;
    error: string | null;
  }>({ load: null, attempt: -1, result: null, error: null });

  useEffect(() => {
    const controller = new AbortController();

    load(controller.signal)
      .then((result) => {
        if (!controller.signal.aborted) {
          setState({ load, attempt, result, error: null });
        }
      })
      .catch((reason: unknown) => {
        if (!controller.signal.aborted) {
          const error = reason instanceof Error ? reason.message : errorMessage;
          setState({ load, attempt, result: null, error });
        }
      });

    return () => {
      controller.abort();
    };
  }, [load, attempt, errorMessage]);

  const settled = state.load === load && state.attempt === attempt;
  const result = settled ? state.result : null;
  const error = settled ? state.error : null;

  const retry = () => {
    setAttempt((current) => current + 1);
  };

  return { result, error, loading: !settled, retry };
}
