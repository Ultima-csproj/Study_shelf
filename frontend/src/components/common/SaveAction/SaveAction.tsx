import { LockKeyIcon } from "@phosphor-icons/react/dist/csr/LockKey";
import { BookmarkSimpleIcon } from "@phosphor-icons/react/dist/csr/BookmarkSimple";
import { useEffect, useState, type ReactNode } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import { useViewer } from "../../../hooks/useViewer";
import type { MaterialSource } from "../../../types/material";
import styles from "./SaveAction.module.css";

interface SaveActionProps {
  id: number;
  title: string;
  compact?: boolean;
  kind?: "material" | "collection";
  outlined?: boolean;
  source?: MaterialSource;
}

export function SaveAction({
  id,
  title,
  compact = false,
  kind = "material",
  outlined = false,
  source = "api",
}: SaveActionProps) {
  const {
    viewer,
    savedStates,
    retrySave,
    registerSave,
    libraryVersion,
    toggleSaved,
    toggleSavedCollection,
  } = useViewer();
  const location = useLocation();
  const navigate = useNavigate();
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const guest = viewer.kind !== "authenticated";
  const collection = kind === "collection";
  const key = `${kind}:${id}`;
  const state = savedStates[key];
  const saved = state?.status === "ready" && state.saved;
  const checking =
    viewer.kind === "loading" ||
    (!guest && (!state || state.status === "loading"));
  const failed = !guest && state?.status === "error";
  useEffect(() => {
    if (viewer.kind === "authenticated" && source === "api") {
      registerSave(kind, id);
    }
  }, [viewer.kind, kind, id, source, registerSave, libraryVersion]);

  const classNames = [styles.action];

  if (guest) {
    classNames.push(styles.locked);
  }

  if (compact) {
    classNames.push(styles.compact);
  }

  if (outlined) {
    classNames.push(styles.outlined);
  }

  const className = classNames.join(" ");
  let action = "Сохранить";
  let buttonTitle = action;
  let buttonLabel = "Сохранить";
  let pressed: boolean | undefined =
    state?.status === "ready" ? saved : undefined;
  let iconWeight: "fill" | "regular" = "regular";

  if (collection) {
    buttonLabel = "Сохранить подборку";
  }

  if (checking) {
    action = "Проверяем сохранение";
    buttonLabel = "Проверяем…";
    buttonTitle = action;
  } else if (failed) {
    action = "Повторить проверку сохранения";
    buttonLabel = "Повторить";
    buttonTitle = action;
  } else if (guest) {
    action = "Войти, чтобы сохранить";
    buttonTitle = "Сохранение — после входа";
    pressed = undefined;
  } else if (saved) {
    action = "Убрать из сохранённого";
    buttonLabel = "Сохранено";
    iconWeight = "fill";
  }

  if (!guest) {
    buttonTitle = action;
  }

  let actionIcon: ReactNode;

  if (guest) {
    actionIcon = <LockKeyIcon size={21} aria-hidden="true" />;
  } else {
    actionIcon = (
      <BookmarkSimpleIcon size={21} weight={iconWeight} aria-hidden="true" />
    );
  }

  const handleClick = async () => {
    if (failed) {
      retrySave(kind, id);
      return;
    }
    if (guest) {
      const redirect = location.pathname + location.search + location.hash;
      navigate(`/login?redirect=${encodeURIComponent(redirect)}`);
      return;
    }

    setBusy(true);
    setError(null);
    try {
      if (collection) {
        await toggleSavedCollection(id);
      } else {
        await toggleSaved({ id, source });
      }
    } catch (reason) {
      setError(
        reason instanceof Error ? reason.message : "Не удалось сохранить",
      );
    } finally {
      setBusy(false);
    }
  };

  return (
    <>
      <button
        type="button"
        className={className}
        aria-label={`${action}: ${title}`}
        aria-pressed={pressed}
        title={buttonTitle}
        onClick={handleClick}
        disabled={
          busy ||
          viewer.kind === "loading" ||
          viewer.kind === "error" ||
          checking ||
          (state?.status === "ready" && state.busy)
        }
      >
        {actionIcon}
        <span>{buttonLabel}</span>
      </button>
      {error && <span role="alert">{error}</span>}
      {failed && <span role="alert">{state.message}</span>}
    </>
  );
}
