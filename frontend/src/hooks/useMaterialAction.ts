import { useRef, useState } from "react";

export function useMaterialAction(onChanged: () => void) {
  const [busy, setBusy] = useState(false);
  const [notice, setNotice] = useState<string | null>(null);
  const running = useRef(false);
  const run = async (
    operation: () => Promise<unknown>,
    successMessage = "Изменения сохранены.",
  ) => {
    if (running.current) {
      return false;
    }
    running.current = true;
    setBusy(true);
    setNotice(null);
    try {
      await operation();
      setNotice(successMessage);
      onChanged();
      return true;
    } catch (reason) {
      setNotice(
        reason instanceof Error
          ? reason.message
          : "Не удалось выполнить действие",
      );
      return false;
    } finally {
      running.current = false;
      setBusy(false);
    }
  };
  return { busy, notice, run };
}
