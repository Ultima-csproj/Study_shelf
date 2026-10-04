CREATE FUNCTION guard_revision_child() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE parent_id BIGINT; parent_state TEXT;
BEGIN
 IF TG_OP = 'DELETE' THEN parent_id := OLD.revision_id; ELSE parent_id := NEW.revision_id; END IF;
 IF TG_OP = 'UPDATE' AND NEW.revision_id <> OLD.revision_id THEN
  RAISE EXCEPTION 'cannot move revision content' USING ERRCODE = '23514';
 END IF;
 SELECT state INTO parent_state FROM material_revisions WHERE id = parent_id FOR UPDATE;
 IF parent_state IS DISTINCT FROM 'draft' THEN
  RAISE EXCEPTION 'revision content is frozen' USING ERRCODE = '23514';
 END IF;
 IF TG_OP = 'DELETE' THEN RETURN OLD; ELSE RETURN NEW; END IF;
END $$;
CREATE TRIGGER revision_credits_guard BEFORE INSERT OR UPDATE OR DELETE ON revision_credits FOR EACH ROW EXECUTE FUNCTION guard_revision_child();
CREATE TRIGGER revision_tags_guard BEFORE INSERT OR UPDATE OR DELETE ON revision_tags FOR EACH ROW EXECUTE FUNCTION guard_revision_child();
CREATE TRIGGER revision_editors_guard BEFORE INSERT OR UPDATE OR DELETE ON revision_editors FOR EACH ROW EXECUTE FUNCTION guard_revision_child();

CREATE FUNCTION guard_revision_snapshot() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF TG_OP = 'DELETE' THEN
  IF OLD.state <> 'draft' THEN RAISE EXCEPTION 'revision is frozen' USING ERRCODE = '23514'; END IF;
  RETURN OLD;
 END IF;
 IF ROW(NEW.id,NEW.material_id,NEW.base_revision_id,NEW.created_by_user_id,NEW.created_at)
  IS DISTINCT FROM ROW(OLD.id,OLD.material_id,OLD.base_revision_id,OLD.created_by_user_id,OLD.created_at) THEN
  RAISE EXCEPTION 'revision identity is immutable' USING ERRCODE = '23514';
 END IF;
 IF OLD.state <> 'draft' AND ROW(NEW.title,NEW.description,NEW.material_type,NEW.source_year,NEW.preview_url,NEW.duration_seconds,NEW.file_size_bytes,NEW.submitted_by_user_id,NEW.submitted_at)
  IS DISTINCT FROM ROW(OLD.title,OLD.description,OLD.material_type,OLD.source_year,OLD.preview_url,OLD.duration_seconds,OLD.file_size_bytes,OLD.submitted_by_user_id,OLD.submitted_at) THEN
  RAISE EXCEPTION 'revision snapshot is immutable' USING ERRCODE = '23514';
 END IF;
 IF NOT ((OLD.state = 'draft' AND NEW.state IN ('draft','pending_moderation'))
  OR (OLD.state = 'pending_moderation' AND NEW.state IN ('published','rejected'))) THEN
  RAISE EXCEPTION 'invalid revision transition' USING ERRCODE = '23514';
 END IF;
 RETURN NEW;
END $$;
CREATE TRIGGER revision_snapshot_guard BEFORE UPDATE OR DELETE ON material_revisions FOR EACH ROW EXECUTE FUNCTION guard_revision_snapshot();

CREATE FUNCTION check_publication_pointer() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF EXISTS (SELECT 1 FROM materials m JOIN material_revisions r ON r.id = m.published_revision_id
 WHERE m.id = NEW.id AND (r.state <> 'published' OR r.published_at < m.first_published_at OR r.created_at < m.created_at)) THEN
  RAISE EXCEPTION 'invalid published revision pointer' USING ERRCODE = '23514';
 END IF;
 RETURN NULL;
END $$;
CREATE CONSTRAINT TRIGGER publication_pointer_guard AFTER INSERT OR UPDATE ON materials
 DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION check_publication_pointer();
CREATE FUNCTION guard_moderation_action() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN RAISE EXCEPTION 'moderation history is immutable' USING ERRCODE = '23514'; END $$;
CREATE TRIGGER moderation_action_guard BEFORE UPDATE OR DELETE ON moderation_actions FOR EACH ROW EXECUTE FUNCTION guard_moderation_action();
