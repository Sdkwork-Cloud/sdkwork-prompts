export interface AdminPromptBindingUpdateRequest {
  promptVersionId?: unknown;
  ownerType?: string;
  ownerId?: string;
  bindingRole?: string;
  priority?: number;
  enabled?: boolean;
  policyJson?: Record<string, unknown>;
}
