import { useOutletContext } from "react-router-dom";
import type { AdminAccess } from "../api/admin";
export function useAdminAccess() {
  return useOutletContext<AdminAccess>();
}
