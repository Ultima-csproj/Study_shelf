export function hasAdmin(user: {
  permissions: string[];
  memberships: { state: string; permissions: string[] }[];
}) {
  return (
    user.permissions.some((p) => p !== "view_account_private_data") ||
    user.memberships.some(
      (m) => m.state === "active" && m.permissions.some((permission) =>
        ["manage_members", "restrict_materials", "review_restricted_materials"].includes(permission)),
    )
  );
}
