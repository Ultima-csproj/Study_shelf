import { useLocation } from "react-router-dom";
import {
  locationPath,
  returnDestination,
  withNavigationOrigin,
} from "../utils/navigation";

/** Keep the exact entry route in the URL, including after a reload or opening a new tab. */
export function useNavigationContext(fallback = "/profile") {
  const location = useLocation();
  const currentPath = locationPath(location);
  const back = returnDestination(location.search, fallback);
  const linkTo = (path: string) => withNavigationOrigin(path, currentPath);
  return { back, linkTo, currentPath };
}
