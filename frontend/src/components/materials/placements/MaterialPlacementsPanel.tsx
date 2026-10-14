import { useCallback, useState } from "react";
import { Link } from "react-router-dom";
import {
  detachEntry,
  detachPlacement,
  getPlacementEntries,
  type MaterialHeader,
  type Placement,
} from "../../../api/materialWorkspace";
import { PagedPanel } from "../shared/PagedPanel";
import styles from "../shared/Panel.module.css";

interface MaterialPlacementsPanelProps {
  material: MaterialHeader;
  placements: Placement[];
  busy: boolean;
  reason: string;
  run: (operation: () => Promise<unknown>) => Promise<boolean>;
}

interface PlacementPanelProps {
  material: MaterialHeader;
  placement: Placement;
  busy: boolean;
  reason: string;
  run: (operation: () => Promise<unknown>) => Promise<boolean>;
}

function PlacementPanel({
  material,
  placement,
  busy,
  reason,
  run,
}: PlacementPanelProps) {
  const [expanded, setExpanded] = useState(false);
  const ready = !busy && reason.trim().length >= 3;
  const load = useCallback(
    (page: number, signal: AbortSignal) =>
      getPlacementEntries(material.id, placement.id, page, signal),
    [material.id, placement.id],
  );
  const detach = () => {
    const confirmation =
      "Отсоединить размещение и все связанные учебные пункты (" +
      placement.entryCount +
      ")? Материал и редакции сохранятся.";
    if (window.confirm(confirmation)) {
      void run(() => detachPlacement(material, placement.id, reason));
    }
  };
  return (
    <div>
      <p>
        <Link to={"/disciplines/" + placement.disciplineId}>
          {placement.disciplineName}
        </Link>{" "}
        · {placement.institutionName} · Пунктов: {placement.entryCount}
      </p>
      <div className={styles.actions}>
        <button
          type="button"
          aria-expanded={expanded}
          onClick={() => setExpanded(!expanded)}
        >
          {expanded ? "Скрыть пункты" : "Показать пункты"}
        </button>
        {placement.canDetach && (
          <button type="button" disabled={!ready} onClick={detach}>
            Отсоединить размещение
          </button>
        )}
      </div>
      {expanded && (
        <PagedPanel
          title="Пункты дисциплины"
          load={load}
          render={(entry) => (
            <p>
              {entry.sectionTitle} · Пункт {entry.position}
              {placement.canDetach && (
                <button
                  type="button"
                  disabled={!ready}
                  onClick={() =>
                    void run(() => detachEntry(material, entry.id, reason))
                  }
                >
                  Отсоединить только этот пункт
                </button>
              )}
            </p>
          )}
        />
      )}
    </div>
  );
}

export function MaterialPlacementsPanel({
  material,
  placements,
  busy,
  reason,
  run,
}: MaterialPlacementsPanelProps) {
  if (placements.length === 0) {
    return null;
  }
  return (
    <section data-material-panel className={styles.card}>
      <h2>Учебные размещения</h2>
      {placements.map((placement) => (
        <PlacementPanel
          key={placement.id + ":" + material.version}
          material={material}
          placement={placement}
          busy={busy}
          reason={reason}
          run={run}
        />
      ))}
    </section>
  );
}
