m.published_revision_id IS NOT NULL
AND (
  m.visibility = 'public'
  OR m.owner_user_id = $1
  OR EXISTS (
    SELECT
      1
    FROM
      institution_memberships im
      JOIN membership_permissions ip USING (user_id, institution_id)
    WHERE
      im.user_id = $1
      AND im.institution_id = m.owner_institution_id
      AND im.state = 'active'
      AND ip.permission = 'manage_materials'
  )
  OR EXISTS (
    SELECT
      1
    FROM
      material_collaborators mc
      JOIN collaborator_permissions cp USING (user_id, material_id)
    WHERE
      mc.user_id = $1
      AND mc.material_id = m.id
      AND mc.active
      AND cp.permission IN (
        'read',
        'edit_revision',
        'submit_for_moderation',
        'manage_collaborators'
      )
  )
)
