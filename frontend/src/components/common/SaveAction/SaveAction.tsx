import { LockKeyIcon } from "@phosphor-icons/react/dist/csr/LockKey";
import { BookmarkSimpleIcon } from "@phosphor-icons/react/dist/csr/BookmarkSimple";
import type { ReactNode } from "react";
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
  source = "mock",
}: SaveActionProps) {
  const {
    viewer,
    savedMaterials,
    savedCollectionIds,
    toggleSaved,
    toggleSavedCollection,
  } = useViewer();
  const location = useLocation();
  const navigate = useNavigate();
  const guest = viewer.kind === "guest";
  const collection = kind === "collection";
  let saved = false;

  if (collection) {
    saved = savedCollectionIds.includes(id);
  } else {
    saved = savedMaterials.some((material) => {
      return material.id === id && material.source === source;
    });
  }

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
  let pressed: boolean | undefined = saved;
  let iconWeight: "fill" | "regular" = "regular";

  if (collection) {
    buttonLabel = "Сохранить подборку";
  }

  if (guest) {
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
      <BookmarkSimpleIcon
        size={21}
        weight={iconWeight}
        aria-hidden="true"
      />
    );
  }

  const handleClick = () => {
    if (guest) {
      const redirect = location.pathname + location.search + location.hash;
      navigate(`/login?redirect=${encodeURIComponent(redirect)}`);
      return;
    }

    if (collection) {
      toggleSavedCollection(id);
    } else {
      toggleSaved({ id, source });
    }
  };

  return (
    <button
      type="button"
      className={className}
      aria-label={`${action}: ${title}`}
      aria-pressed={pressed}
      title={buttonTitle}
      onClick={handleClick}
    >
      {actionIcon}
      <span>{buttonLabel}</span>
    </button>
  );
}
