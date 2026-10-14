import type { CollectionSectionView } from "../../../types/collection";
import { getCollectionSectionItems } from "../../../api/collections";
import { useSectionItems } from "../../../hooks/useSectionItems";
import { CollectionRouteItem } from "../CollectionRouteItem/CollectionRouteItem";
import { Button } from "../../common/Button/Button";

interface CollectionSectionEntriesProps {
  collectionId: number;
  section: CollectionSectionView;
  sectionId: string;
}
export function CollectionSectionEntries({ collectionId, section, sectionId }: CollectionSectionEntriesProps) {
  const load = (page: number, signal: AbortSignal) => {
    return getCollectionSectionItems(collectionId, section.id, page, section.initialPageSize, signal);
  };
  const { items, loading, error, hasMore, loadMore } = useSectionItems(section.items, section.stepCount, load);
  return (
    <>
      <ol aria-label={`Материалы раздела «${section.title}»`}>
        {items.map((item) => (
          <CollectionRouteItem
            key={item.id}
            item={item}
            sectionNumber={section.position}
            sectionId={sectionId}
          />
        ))}
      </ol>
      {error && <p role="alert">{error}</p>}
      {hasMore && (
        <Button variant="secondary" onClick={loadMore} disabled={loading}>
          {loading ? "Загрузка…" : "Показать ещё"}
        </Button>
      )}
    </>
  );
}
