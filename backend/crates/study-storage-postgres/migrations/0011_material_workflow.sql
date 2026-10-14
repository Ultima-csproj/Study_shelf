ALTER TABLE material_revisions
    DROP CONSTRAINT material_revisions_state_check,
    DROP CONSTRAINT material_revisions_check4;

ALTER TABLE material_revisions
    ADD CONSTRAINT material_revisions_state_check
    CHECK (state IN (
        'draft',
        'pending_moderation',
        'published',
        'rejected',
        'discarded'
    )),
    ADD CONSTRAINT material_revision_lifecycle_check
    CHECK (
        (
            state IN ('draft', 'discarded')
            AND submitted_by_user_id IS NULL
            AND submitted_at IS NULL
            AND published_at IS NULL
        )
        OR (
            state IN ('pending_moderation', 'rejected')
            AND submitted_by_user_id IS NOT NULL
            AND submitted_at IS NOT NULL
            AND published_at IS NULL
        )
        OR (
            state = 'published'
            AND submitted_by_user_id IS NOT NULL
            AND submitted_at IS NOT NULL
            AND published_at IS NOT NULL
        )
    );

ALTER TABLE materials
    ADD COLUMN lifecycle TEXT NOT NULL DEFAULT 'active'
        CHECK (lifecycle IN ('active', 'archived')),
    ADD COLUMN publication_state TEXT NOT NULL DEFAULT 'unpublished'
        CHECK (publication_state IN ('published', 'unpublished')),
    ADD COLUMN edit_version BIGINT NOT NULL DEFAULT 1
        CHECK (edit_version > 0);

ALTER TABLE materials
    ADD CONSTRAINT material_publication_state_check
        CHECK (publication_state <> 'published' OR published_revision_id IS NOT NULL);

UPDATE materials
SET publication_state = 'published'
WHERE published_revision_id IS NOT NULL;

ALTER TABLE user_platform_permissions
    DROP CONSTRAINT user_platform_permissions_permission_check;

ALTER TABLE user_platform_permissions
    ADD CONSTRAINT user_platform_permissions_permission_check
    CHECK (permission IN (
        'moderate_public_materials',
        'manage_accounts',
        'manage_institutions',
        'manage_platform_permissions',
        'view_account_private_data',
        'restrict_public_materials',
        'review_restricted_materials'
    ));

INSERT INTO user_platform_permissions (
    user_id,
    permission
)
SELECT
    p.user_id,
    capability.permission
FROM user_platform_permissions AS p
CROSS JOIN unnest(ARRAY[
    'restrict_public_materials',
    'review_restricted_materials'
]) AS capability(permission)
WHERE p.permission = 'manage_platform_permissions'
ON CONFLICT DO NOTHING;

UPDATE users
SET admin_version = admin_version + 1
WHERE EXISTS (
    SELECT 1
    FROM user_platform_permissions
    WHERE user_id = users.id
        AND permission = 'manage_platform_permissions'
);

ALTER TABLE membership_permissions
    DROP CONSTRAINT membership_permissions_permission_check;

ALTER TABLE membership_permissions
    ADD CONSTRAINT membership_permissions_permission_check
    CHECK (permission IN (
        'manage_members',
        'manage_disciplines',
        'create_materials',
        'manage_materials',
        'restrict_materials',
        'review_restricted_materials'
    ));

INSERT INTO membership_permissions (
    user_id,
    institution_id,
    permission
)
SELECT
    full_manager.user_id,
    full_manager.institution_id,
    capability.permission
FROM (
    SELECT
        user_id,
        institution_id
    FROM membership_permissions
    WHERE permission IN (
        'manage_members',
        'manage_disciplines',
        'create_materials',
        'manage_materials'
    )
    GROUP BY user_id, institution_id
    HAVING count(*) = 4
) AS full_manager
CROSS JOIN unnest(ARRAY[
    'restrict_materials',
    'review_restricted_materials'
]) AS capability(permission)
ON CONFLICT DO NOTHING;

UPDATE institution_memberships AS membership
SET edit_version = edit_version + 1
WHERE EXISTS (
    SELECT 1
    FROM membership_permissions AS permission
    WHERE permission.user_id = membership.user_id
        AND permission.institution_id = membership.institution_id
        AND permission.permission = 'restrict_materials'
);

CREATE TABLE material_restrictions (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    material_id BIGINT NOT NULL REFERENCES materials(id),
    institution_id BIGINT REFERENCES institutions(id),
    created_by_user_id BIGINT NOT NULL REFERENCES users(id),
    reason TEXT NOT NULL CHECK (length(btrim(reason)) BETWEEN 3 AND 2000),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    revoked_by_user_id BIGINT REFERENCES users(id),
    revoked_at TIMESTAMPTZ,
    revoke_reason TEXT,
    CHECK ((revoked_at IS NULL) = (revoked_by_user_id IS NULL)),
    CHECK ((revoked_at IS NULL) = (revoke_reason IS NULL))
);

CREATE UNIQUE INDEX material_restrictions_active_scope_idx
    ON material_restrictions (material_id, institution_id) NULLS NOT DISTINCT
    WHERE revoked_at IS NULL;

CREATE INDEX material_restrictions_scope_idx
    ON material_restrictions (institution_id, material_id)
    WHERE revoked_at IS NULL;

CREATE UNIQUE INDEX material_working_revision_idx
    ON material_revisions (material_id)
    WHERE state IN ('draft', 'pending_moderation');

CREATE INDEX material_moderation_queue_idx
    ON material_revisions (submitted_at, id)
    WHERE state = 'pending_moderation';

CREATE TABLE material_events (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    material_id BIGINT NOT NULL REFERENCES materials(id),
    revision_id BIGINT REFERENCES material_revisions(id),
    actor_user_id BIGINT NOT NULL REFERENCES users(id),
    action TEXT NOT NULL,
    reason TEXT NOT NULL CHECK (length(btrim(reason)) BETWEEN 3 AND 2000),
    details JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX material_events_history_idx
    ON material_events (material_id, id DESC);

CREATE TRIGGER material_events_immutable
    BEFORE UPDATE OR DELETE ON material_events
    FOR EACH ROW EXECUTE FUNCTION guard_moderation_action();

CREATE FUNCTION material_publicly_readable (
    target_material BIGINT,
    context_institution BIGINT DEFAULT NULL
)
RETURNS BOOLEAN
LANGUAGE sql
STABLE
AS $$
    SELECT EXISTS (
        SELECT 1
        FROM materials AS m
        WHERE m.id = target_material
            AND m.visibility = 'public'
            AND m.publication_state = 'published'
            AND m.published_revision_id IS NOT NULL
            AND NOT EXISTS (
                SELECT 1
                FROM material_restrictions AS restriction
                WHERE restriction.material_id = m.id
                    AND restriction.revoked_at IS NULL
                    AND (
                        restriction.institution_id IS NULL
                        OR restriction.institution_id = context_institution
                    )
            )
    );
$$;

CREATE OR REPLACE VIEW public_materials AS
SELECT
    m.id,
    r.id AS published_revision_id,
    r.title,
    r.description,
    r.material_type,
    r.source_year,
    r.preview_url,
    r.duration_seconds,
    r.file_size_bytes,
    m.first_published_at,
    r.published_at,
    m.lifecycle
FROM materials AS m
JOIN material_revisions AS r
    ON r.id = m.published_revision_id
    AND r.material_id = m.id
WHERE material_publicly_readable(m.id)
    AND r.state = 'published';

CREATE OR REPLACE FUNCTION guard_revision_snapshot()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
    IF TG_OP = 'DELETE' THEN
        IF OLD.state <> 'draft' THEN
            RAISE EXCEPTION 'revision is frozen' USING ERRCODE = '23514';
        END IF;
        RETURN OLD;
    END IF;

    IF ROW(
        NEW.id,
        NEW.material_id,
        NEW.base_revision_id,
        NEW.created_by_user_id,
        NEW.created_at
    ) IS DISTINCT FROM ROW(
        OLD.id,
        OLD.material_id,
        OLD.base_revision_id,
        OLD.created_by_user_id,
        OLD.created_at
    ) THEN
        RAISE EXCEPTION 'revision identity is immutable' USING ERRCODE = '23514';
    END IF;

    IF NOT (
        (OLD.state = 'draft' AND NEW.state IN ('draft', 'pending_moderation', 'discarded'))
        OR (OLD.state = 'pending_moderation' AND NEW.state IN ('draft', 'published', 'rejected'))
    ) THEN
        RAISE EXCEPTION 'invalid revision transition' USING ERRCODE = '23514';
    END IF;

    IF OLD.state <> 'draft'
        AND (to_jsonb(NEW) - ARRAY[
            'state', 'updated_at', 'published_at', 'edit_version',
            'submitted_by_user_id', 'submitted_at'
        ]) IS DISTINCT FROM (to_jsonb(OLD) - ARRAY[
            'state', 'updated_at', 'published_at', 'edit_version',
            'submitted_by_user_id', 'submitted_at'
        ]) THEN
        RAISE EXCEPTION 'revision snapshot is immutable' USING ERRCODE = '23514';
    END IF;

    IF OLD.state = 'pending_moderation' AND NEW.state <> 'draft'
        AND ROW(NEW.submitted_by_user_id, NEW.submitted_at)
            IS DISTINCT FROM ROW(OLD.submitted_by_user_id, OLD.submitted_at) THEN
        RAISE EXCEPTION 'submission is immutable' USING ERRCODE = '23514';
    END IF;

    RETURN NEW;
END;
$$;

CREATE FUNCTION material_readable_by (
    target_material BIGINT,
    viewer_user BIGINT,
    context_institution BIGINT DEFAULT NULL
)
RETURNS BOOLEAN
LANGUAGE sql
STABLE
AS $$
    SELECT EXISTS (
        SELECT 1
        FROM materials AS m
        WHERE m.id = target_material
            AND m.published_revision_id IS NOT NULL
            AND (
                material_publicly_readable(m.id, context_institution)
                OR (
                    EXISTS (
                        SELECT 1
                        FROM users
                        WHERE id = viewer_user
                            AND state = 'active'
                    )
                    AND (
                        (
                            NOT EXISTS (
                                SELECT 1
                                FROM material_restrictions
                                WHERE material_id = m.id
                                    AND revoked_at IS NULL
                                    AND (
                                        institution_id IS NULL
                                        OR institution_id = context_institution
                                    )
                            )
                            AND (
                                m.owner_user_id = viewer_user
                                OR EXISTS (
                                    SELECT 1
                                    FROM institution_memberships AS membership
                                    JOIN membership_permissions AS permission
                                        USING (user_id, institution_id)
                                    WHERE membership.user_id = viewer_user
                                        AND membership.institution_id = m.owner_institution_id
                                        AND membership.state = 'active'
                                        AND (
                                            permission.permission = 'manage_materials'
                                            OR (
                                                m.created_by_user_id = viewer_user
                                                AND permission.permission = 'create_materials'
                                            )
                                        )
                                )
                                OR EXISTS (
                                    SELECT 1
                                    FROM material_collaborators AS collaborator
                                    JOIN collaborator_permissions AS permission
                                        USING (user_id, material_id)
                                    WHERE collaborator.material_id = m.id
                                        AND collaborator.user_id = viewer_user
                                        AND collaborator.active
                                )
                            )
                        )
                        OR (
                            m.visibility = 'public'
                            AND (
                                EXISTS (
                                    SELECT 1
                                    FROM user_platform_permissions
                                    WHERE user_id = viewer_user
                                        AND permission = 'review_restricted_materials'
                                )
                                OR (
                                    NOT EXISTS (
                                        SELECT 1
                                        FROM material_restrictions
                                        WHERE material_id = m.id
                                            AND institution_id IS NULL
                                            AND revoked_at IS NULL
                                    )
                                    AND EXISTS (
                                        SELECT 1
                                        FROM institution_memberships AS membership
                                        JOIN membership_permissions AS permission
                                            USING (user_id, institution_id)
                                        WHERE membership.user_id = viewer_user
                                            AND membership.institution_id = context_institution
                                            AND membership.state = 'active'
                                            AND permission.permission = 'review_restricted_materials'
                                    )
                                )
                            )
                        )
                    )
                )
            )
    );
$$;

