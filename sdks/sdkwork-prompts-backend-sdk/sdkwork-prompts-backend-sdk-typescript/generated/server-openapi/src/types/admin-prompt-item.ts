export interface AdminPromptItem {
  id: string;
  uuid: string;
  tenantId: string;
  organizationId: string;
  promptKey: string;
  name: string;
  description?: string | null;
  categoryId?: string | null;
  categoryCode?: string | null;
  promptType: string;
  visibility: string;
  status: string;
  tags: string[];
  ownerUserId?: string | null;
  latestVersionId?: string | null;
  publishedVersionId?: string | null;
  createdAt: string;
  updatedAt: string;
}
