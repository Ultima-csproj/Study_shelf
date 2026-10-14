import type { Page } from "../library";
import type { AdminQuery } from "../admin";
import type { ContributorRole, MaterialType } from "../../types/material";

export interface MaterialHeader {
  id: number;
  title: string;
  lifecycle: "active" | "archived";
  publication: "published" | "unpublished";
  visibility: "public" | "private";
  version: number;
  ownerUserId: number | null;
  ownerInstitutionId: number | null;
  ownerName: string;
  publishedRevisionId: number | null;
  workingRevisionId: number | null;
  workingRevisionState: string | null;
  materialType: MaterialType | null;
  sourceYear: number | null;
  description: string | null;
  updatedAt: string | null;
  publishedRevisionNumber: number | null;
}

export interface RevisionHeader {
  id: number;
  number: number;
  title: string;
  state: string;
  version: number;
  baseRevisionId: number | null;
  createdAt: string;
  publishedAt: string | null;
}

export interface CreditInput {
  contributorId: number;
  role: ContributorRole;
}

export interface ContentInput {
  title: string;
  description: string;
  materialType: MaterialType;
  sourceYear: number | null;
  previewUrl: string | null;
  durationSeconds: number | null;
  fileSizeBytes: number | null;
  tags: string[];
  credits: CreditInput[];
}

export interface Revision extends ContentInput {
  id: number;
  number: number;
  materialId: number;
  version: number;
  state: string;
  baseRevisionId: number | null;
  createdAt: string;
  updatedAt: string;
  submittedAt: string | null;
  publishedAt: string | null;
  credits: (CreditInput & { displayName: string })[];
}

export interface Restriction {
  id: number;
  institutionId: number | null;
  institutionName: string | null;
  reason: string;
  createdAt: string;
}

export interface RestrictionTarget {
  material: MaterialHeader;
  global: boolean;
  institutions: number[];
}

export interface Placement {
  id: number;
  disciplineId: number;
  disciplineName: string;
  institutionId: number;
  institutionName: string;
  entryCount: number;
  canDetach: boolean;
}

export interface PlacementEntry {
  id: number;
  sectionId: number;
  sectionTitle: string;
  position: number;
}

export interface Workspace {
  material: MaterialHeader;
  revisions: Page<RevisionHeader>;
  restrictions: Restriction[];
  placements: Placement[];
  capabilities: {
    manage: boolean;
    edit: boolean;
    submit: boolean;
    collaborators: boolean;
    restrictGlobal: boolean;
    restrictInstitutions: number[];
  };
}

export interface RevisionResponse {
  material: MaterialHeader;
  revision: Revision;
  outdated: boolean;
  selfReview: boolean;
  canModerate: boolean;
  decision: {
    reviewerId: number;
    decision: string;
    reason: string | null;
    reviewedAt: string;
    wasSelfReview: boolean;
  } | null;
}

export interface Collaborator {
  userId: number;
  name: string;
  active: boolean;
  permissions: string[];
}

export interface MaterialEvent {
  id: number;
  revisionId: number | null;
  actorName: string;
  action: string;
  reason: string;
  createdAt: string;
}

export interface PublishedHistory {
  materialId: number;
  lifecycle: string;
  currentRevisionId: number | null;
  revisions: Page<RevisionHeader>;
}

export interface AuthorQuery extends AdminQuery {
  order?: "newest" | "oldest" | "title";
}

export interface ContributorChoice {
  id: number;
  displayName: string;
}

export const revisionLabels: Record<string, string> = {
  draft: "Черновик",
  pending_moderation: "На модерации",
  published: "Опубликована",
  rejected: "Отклонена",
  discarded: "Черновик отменён",
};

export const collaboratorLabels: Record<string, string> = {
  read: "Читать материал и редакции",
  edit_revision: "Редактировать черновик",
  submit_for_moderation: "Отправлять на модерацию",
  manage_collaborators: "Управлять приглашёнными",
};

export const creditLabels: Record<ContributorRole, string> = {
  author: "Автор",
  co_author: "Соавтор",
  editor: "Редактор",
  translator: "Переводчик",
  lecturer: "Лектор",
  reviewer: "Рецензент",
};
