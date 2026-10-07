ALTER TABLE collections
ADD COLUMN edit_version BIGINT NOT NULL DEFAULT 1 CHECK (edit_version > 0);

ALTER TABLE collection_sections
DROP CONSTRAINT collection_sections_collection_id_position_key;

ALTER TABLE collection_sections
ADD CONSTRAINT collection_sections_collection_id_position_key UNIQUE (collection_id, position)
DEFERRABLE INITIALLY DEFERRED;

ALTER TABLE collection_entries
DROP CONSTRAINT collection_entries_section_id_position_key;

ALTER TABLE collection_entries
ADD CONSTRAINT collection_entries_section_id_position_key UNIQUE (section_id, position)
DEFERRABLE INITIALLY DEFERRED;
