// Public API contract; implementations are grouped by responsibility.
export type {
  MaterialHeader,
  RevisionHeader,
  CreditInput,
  ContentInput,
  Revision,
  Restriction,
  RestrictionTarget,
  Placement,
  PlacementEntry,
  Workspace,
  RevisionResponse,
  Collaborator,
  MaterialEvent,
  PublishedHistory,
  AuthorQuery,
  ContributorChoice,
} from "./materialWorkspace/types";
export { revisionLabels, collaboratorLabels, creditLabels } from "./materialWorkspace/types";
export {
  getAuthorMaterials,
  getModerationMaterials,
  getRestrictedMaterials,
  getRestrictionMetadata,
  getRestrictionTarget,
  getWorkspace,
  getRevision,
  findMaterialContributors,
  getCollaborators,
  getMaterialEvents,
} from "./materialWorkspace/queries";
export {
  createMaterial,
  saveRevision,
  forkRevision,
  transitionRevision,
} from "./materialWorkspace/revisions";
export {
  changeMaterialState,
  restrictMaterial,
  unrestrictMaterial,
  setCollaborator,
} from "./materialWorkspace/access";
export {
  detachPlacement,
  getPlacementEntries,
  detachEntry,
} from "./materialWorkspace/placements";
export { getPublishedHistory, getPublishedRevision } from "./materialWorkspace/history";
