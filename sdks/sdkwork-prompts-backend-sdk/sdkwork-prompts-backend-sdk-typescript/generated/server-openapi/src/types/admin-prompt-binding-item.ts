export interface AdminPromptBindingItem {
  id: string;
  uuid: string;
  tenantId: string;
  organizationId: string;
  promptId: string;
  promptVersionId?: string | null;
  ownerType: string;
  ownerId: string;
  bindingRole: string;
  priority: number;
  enabled: boolean;
  policyJson?: Record<string, unknown>;
  snapshotJson?: Record<string, unknown>;
  createdAt: string;
  updatedAt: string;
}
