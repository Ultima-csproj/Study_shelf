CREATE TABLE user_credentials (
  user_id BIGINT PRIMARY KEY REFERENCES users (id),
  email TEXT NOT NULL UNIQUE CHECK (
    email = lower(btrim(email))
    AND length(email) BETWEEN 3 AND 254
  ),
  password_hash TEXT NOT NULL,
  email_verified_at TIMESTAMPTZ,
  password_version BIGINT NOT NULL DEFAULT 1 CHECK (password_version > 0)
);

CREATE TABLE user_profiles (
  user_id BIGINT PRIMARY KEY REFERENCES users (id),
  first_name TEXT NOT NULL CHECK (length(btrim(first_name)) BETWEEN 1 AND 64),
  last_name TEXT NOT NULL DEFAULT '' CHECK (length(last_name) <= 64),
  course SMALLINT CHECK (course BETWEEN 1 AND 6),
  study_group TEXT NOT NULL DEFAULT '' CHECK (length(study_group) <= 30),
  biography TEXT NOT NULL DEFAULT '' CHECK (length(biography) <= 2000),
  primary_contributor_id BIGINT NOT NULL REFERENCES contributors (id)
);

CREATE TABLE user_sessions (
  token_hash BYTEA PRIMARY KEY CHECK (octet_length(token_hash) = 32),
  user_id BIGINT NOT NULL REFERENCES users (id),
  csrf_hash BYTEA NOT NULL CHECK (octet_length(csrf_hash) = 32),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  expires_at TIMESTAMPTZ NOT NULL CHECK (expires_at > created_at),
  revoked_at TIMESTAMPTZ
);

CREATE INDEX user_sessions_user_idx ON user_sessions (user_id)
WHERE
  revoked_at IS NULL;

CREATE INDEX user_sessions_expiry_idx ON user_sessions (expires_at);

CREATE TABLE saved_materials (
  user_id BIGINT NOT NULL REFERENCES users (id),
  material_id BIGINT NOT NULL REFERENCES materials (id),
  saved_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  PRIMARY KEY (user_id, material_id)
);

CREATE INDEX saved_materials_page_idx ON saved_materials (user_id, saved_at DESC, material_id);

CREATE TABLE saved_collections (
  user_id BIGINT NOT NULL REFERENCES users (id),
  collection_id BIGINT NOT NULL REFERENCES collections (id),
  saved_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  PRIMARY KEY (user_id, collection_id)
);

CREATE INDEX saved_collections_page_idx ON saved_collections (user_id, saved_at DESC, collection_id);

ALTER TABLE user_platform_permissions
DROP CONSTRAINT user_platform_permissions_permission_check;

ALTER TABLE user_platform_permissions
ADD CONSTRAINT user_platform_permissions_permission_check CHECK (
  permission IN (
    'moderate_public_materials',
    'manage_accounts',
    'manage_institutions',
    'manage_platform_permissions'
  )
);

CREATE TABLE identity_audit (
  id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  actor_user_id BIGINT REFERENCES users (id),
  subject_user_id BIGINT NOT NULL REFERENCES users (id),
  action TEXT NOT NULL,
  scope_id BIGINT,
  source TEXT NOT NULL CHECK (source IN ('http', 'cli')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE contributor_link_requests (
  user_id BIGINT NOT NULL REFERENCES users (id),
  contributor_id BIGINT NOT NULL REFERENCES contributors (id),
  state TEXT NOT NULL CHECK (state IN ('pending', 'accepted', 'rejected')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  PRIMARY KEY (user_id, contributor_id)
);
