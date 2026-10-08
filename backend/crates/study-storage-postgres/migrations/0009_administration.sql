ALTER TABLE user_platform_permissions
DROP CONSTRAINT user_platform_permissions_permission_check;

ALTER TABLE user_platform_permissions
ADD CONSTRAINT user_platform_permissions_permission_check CHECK (
  permission IN (
    'moderate_public_materials',
    'manage_accounts',
    'manage_institutions',
    'manage_platform_permissions',
    'view_account_private_data'
  )
);

-- Preserve the complete bootstrap administrator bundle, without giving private data to every account manager.
INSERT INTO
  user_platform_permissions (user_id, permission)
SELECT
  user_id,
  'view_account_private_data'
FROM
  user_platform_permissions
WHERE
  permission = 'manage_platform_permissions'
ON CONFLICT DO NOTHING;

ALTER TABLE users
ADD COLUMN admin_version BIGINT NOT NULL DEFAULT 1 CHECK (admin_version > 0);

ALTER TABLE institutions
ADD COLUMN edit_version BIGINT NOT NULL DEFAULT 1 CHECK (edit_version > 0);

ALTER TABLE institution_memberships
ADD COLUMN edit_version BIGINT NOT NULL DEFAULT 1 CHECK (edit_version > 0);

ALTER TABLE institution_memberships
ADD COLUMN requested_at TIMESTAMPTZ;

ALTER TABLE institution_memberships
ADD COLUMN decision_reason TEXT;

ALTER TABLE contributor_link_requests
ADD COLUMN edit_version BIGINT NOT NULL DEFAULT 1 CHECK (edit_version > 0);

ALTER TABLE contributor_link_requests
ADD COLUMN evidence TEXT NOT NULL DEFAULT '' CHECK (length(evidence) <= 2000);

ALTER TABLE contributor_link_requests
ADD COLUMN decision_reason TEXT;

CREATE TABLE institution_registration_requests (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  user_id BIGINT NOT NULL REFERENCES users (id),
  name TEXT NOT NULL CHECK (length(btrim(name)) BETWEEN 1 AND 200),
  description TEXT NOT NULL DEFAULT '' CHECK (length(description) <= 2000),
  evidence TEXT NOT NULL CHECK (length(btrim(evidence)) BETWEEN 3 AND 2000),
  state TEXT NOT NULL DEFAULT 'pending' CHECK (state IN ('pending', 'accepted', 'rejected')),
  edit_version BIGINT NOT NULL DEFAULT 1 CHECK (edit_version > 0),
  institution_id BIGINT REFERENCES institutions (id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  decided_at TIMESTAMPTZ,
  decision_reason TEXT,
  CHECK ((state = 'pending') = (decided_at IS NULL)),
  CHECK (
    (state = 'accepted') = (institution_id IS NOT NULL)
  )
);

CREATE INDEX institution_registration_queue_idx ON institution_registration_requests (state, created_at, id);

CREATE UNIQUE INDEX institution_registration_pending_user_idx ON institution_registration_requests (user_id)
WHERE
  state = 'pending';

CREATE TABLE administration_audit (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  actor_user_id BIGINT REFERENCES users (id),
  target_kind TEXT NOT NULL CHECK (
    target_kind IN (
      'user',
      'membership',
      'contributor_request',
      'institution',
      'institution_request'
    )
  ),
  target_id BIGINT NOT NULL CHECK (target_id > 0),
  institution_id BIGINT REFERENCES institutions (id),
  action TEXT NOT NULL,
  reason TEXT NOT NULL,
  before_data JSONB NOT NULL DEFAULT '{}',
  after_data JSONB NOT NULL DEFAULT '{}',
  source TEXT NOT NULL CHECK (source IN ('http', 'cli')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

INSERT INTO
  administration_audit (
    actor_user_id,
    target_kind,
    target_id,
    institution_id,
    action,
    reason,
    before_data,
    source,
    created_at
  )
SELECT
  actor_user_id,
  'user',
  subject_user_id,
  NULL,
  action,
  'Историческая операция этапа 12',
  jsonb_build_object('legacyAuditId', id, 'scopeId', scope_id),
  source,
  created_at
FROM
  identity_audit;

CREATE INDEX administration_audit_order_idx ON administration_audit (created_at DESC, id DESC);

CREATE INDEX administration_audit_institution_idx ON administration_audit (institution_id, id DESC);
