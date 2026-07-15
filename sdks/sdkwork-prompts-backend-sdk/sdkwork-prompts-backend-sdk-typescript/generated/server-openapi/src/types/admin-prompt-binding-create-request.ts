export interface AdminPromptBindingCreateRequest {
  promptVersionId?: string;
  ownerType: string;
  ownerId: string;
  bindingRole: string;
  priority?: number;
  enabled?: boolean;
  policyJson?: Record<string, unknown>;
}
