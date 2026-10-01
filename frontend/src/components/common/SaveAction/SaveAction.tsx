import { LockKeyIcon } from "@phosphor-icons/react/dist/csr/LockKey";
import { BookmarkSimpleIcon } from "@phosphor-icons/react/dist/csr/BookmarkSimple";
import { useLocation, useNavigate } from "react-router-dom";
import { useViewer } from "../../../hooks/useViewer";
import type { MaterialSource } from "../../../types/material";
import styles from "./SaveAction.module.css";
export function SaveAction({
  id,
  title,
  compact = false,
  kind = "material",
  outlined = false,
  source = "mock",
}: {
  id: number;
  title: string;
  compact?: boolean;
  kind?: "material" | "collection";
  outlined?: boolean;
  source?: MaterialSource;
}) {
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
  const saved = collection
    ? savedCollectionIds.includes(id)
    : savedMaterials.some((material) => {
        return material.id === id && material.source === source;
      });
  const Icon = guest ? LockKeyIcon : BookmarkSimpleIcon;
  const action = guest
    ? "Войти, чтобы сохранить"
    : saved
      ? "Убрать из сохранённого"
      : "Сохранить";
  const className = [
    styles.action,
    guest ? styles.locked : "",
    compact ? styles.compact : "",
    outlined ? styles.outlined : "",
  ].join(" ");
  const buttonTitle = guest ? "Сохранение — после входа" : action;
  const weight = saved && !guest ? "fill" : "regular";
  const unsavedLabel = collection ? "Сохранить подборку" : "Сохранить";
  const label = saved && !guest ? "Сохранено" : unsavedLabel;
  const pressed = guest ? undefined : saved;
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
      <Icon size={21} weight={weight} aria-hidden="true" />
      <span>{label}</span>
    </button>
  );
}
