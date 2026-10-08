import { useCallback } from "react";
import { getMaterialEvents } from "../../api/materialWorkspace";
import { PagedPanel } from "./PagedPanel";
import { exactPublication } from "../../utils/materialLabels";

const labels: Record<string, string> = {
  create_material: "Создание материала",
  create_draft: "Создание черновика",
  save_draft: "Сохранение черновика",
  submit_revision: "Отправка на модерацию",
  withdraw_revision: "Отзыв с модерации",
  discard_draft: "Отмена черновика",
  approve_revision: "Принятие редакции",
  reject_revision: "Отклонение редакции",
  archive_material: "Архивирование",
  restore_material: "Восстановление",
  unpublish_material: "Снятие публикации",
  republish_material: "Возврат публикации",
  make_material_private: "Приватный доступ",
  restrict_material: "Ограничение доступа",
  unrestrict_material: "Снятие ограничения",
  set_collaborator: "Приглашения и разрешения",
  detach_placement: "Отсоединение размещения",
  detach_entry: "Отсоединение учебного пункта",
};
interface MaterialEventsPanelProps {
  id: number;
}
export function MaterialEventsPanel({ id }: MaterialEventsPanelProps) {
  const load = useCallback(
    (page: number, signal: AbortSignal) =>
      getMaterialEvents(id, { page, q: "", state: "" }, signal),
    [id],
  );
  return (
    <PagedPanel
      title="История действий"
      load={load}
      render={(event) => (
        <p>
          {labels[event.action] ?? event.action} · {event.actorName} ·{" "}
          {exactPublication(event.createdAt)}
          <br />
          {event.reason}
        </p>
      )}
    />
  );
}
