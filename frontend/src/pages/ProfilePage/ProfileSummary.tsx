import type { User } from "../../types/viewer";
import { ProfileOverview } from "../../components/account/profile-overview/ProfileOverview";

export function ProfileSummary({
  user,
  onEdit,
}: {
  user: User;
  onEdit: () => void;
}) {
  return <ProfileOverview user={user} onEdit={onEdit} />;
}
