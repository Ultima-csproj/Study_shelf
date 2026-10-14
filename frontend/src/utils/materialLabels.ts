import type { CreditView, ContributorRole } from "../types/material";
import type { MaterialType } from "../types/material";

export const materialTypeLabels: Record<MaterialType, string> = {
  lecture: "Лекция",
  book: "Книга",
  article: "Статья",
  video: "Видео",
  lab: "Лабораторная работа",
};

export const contributorRoles: Record<ContributorRole, string> = {
  author: "Автор",
  co_author: "Соавтор",
  editor: "Редактор",
  translator: "Переводчик",
  lecturer: "Лектор",
  reviewer: "Рецензент",
};
export function authorLabel(credits: readonly CreditView[]): string {
  const authors = credits.filter((credit) => {
    return credit.role === "author" || credit.role === "co_author";
  });
  if (authors.length === 0) {
    return "Автор не указан";
  }
  return authors.map((credit) => credit.displayName).join(", ");
}
export function formatDuration(seconds: number | null | undefined): string | null {
  if (seconds === null || seconds === undefined) {
    return null;
  }
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const remaining = seconds % 60;
  const paddedSeconds = String(remaining).padStart(2, "0");
  if (hours > 0) {
    return hours + ":" + String(minutes).padStart(2, "0") + ":" + paddedSeconds;
  }
  return minutes + ":" + paddedSeconds;
}
const relativeFormatter = new Intl.RelativeTimeFormat("ru", { numeric: "always" });
const timeUnits: [number, Intl.RelativeTimeFormatUnit][] = [
  [31536000, "year"],
  [2592000, "month"],
  [86400, "day"],
  [3600, "hour"],
  [60, "minute"],
  [1, "second"],
];
export function relativePublication(value: string): string {
  const seconds = Math.trunc((new Date(value).getTime() - Date.now()) / 1000);
  for (const [size, unit] of timeUnits) {
    if (Math.abs(seconds) >= size || size === 1) {
      return relativeFormatter.format(Math.trunc(seconds / size), unit);
    }
  }
  return relativeFormatter.format(0, "second");
}
export function publicationLabel(first: string, current: string): string {
  if (first === current) {
    return "Добавлено";
  }
  return "Обновлено";
}
export function exactPublication(value: string): string {
  return new Intl.DateTimeFormat("ru-RU", { dateStyle: "long", timeStyle: "short" }).format(new Date(value));
}
