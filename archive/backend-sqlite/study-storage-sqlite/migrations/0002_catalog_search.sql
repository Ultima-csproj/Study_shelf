-- A normalized projection for the first, read-only catalog. Future write paths
-- must update these columns in the same transaction as the source material.
ALTER TABLE materials
ADD COLUMN search_text TEXT NOT NULL DEFAULT '';

ALTER TABLE materials
ADD COLUMN sort_title TEXT NOT NULL DEFAULT '';

CREATE INDEX idx_materials_sort_title ON materials (sort_title, id);
