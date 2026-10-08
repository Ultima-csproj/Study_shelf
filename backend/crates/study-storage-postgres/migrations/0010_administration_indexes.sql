CREATE INDEX institution_membership_queue_idx ON institution_memberships (institution_id, state, user_id);

CREATE INDEX contributor_link_queue_idx ON contributor_link_requests (state, created_at, user_id, contributor_id);
