export interface PromptsBackendSdkDependencies {
  prompts: PromptsBackendGeneratedClient;
}

export interface PromptsBackendGeneratedClient {
  definitions: {
    list(params?: {
      page?: string;
      pageSize?: string;
      q?: string;
      promptType?: string;
      visibility?: string;
      status?: string;
      categoryId?: string;
    }): Promise<AdminPromptListPage>;
    create(
      body: AdminPromptCreateRequest,
      params: { idempotencyKey: string },
    ): Promise<AdminPromptItem>;
  };
  versions: {
    list(promptId: string): Promise<AdminPromptVersionListPage>;
    create(
      promptId: string,
      body: AdminPromptVersionCreateRequest,
      params: { idempotencyKey: string },
    ): Promise<AdminPromptVersionItem>;
    publish(versionId: string): Promise<AdminPromptVersionItem>;
  };
  versionRenders: {
    create(versionId: string, body: AdminPromptRenderRequest): Promise<AdminPromptRenderResult>;
  };
  definitionBindings: {
    list(promptId: string): Promise<AdminPromptBindingListPage>;
    create(
      promptId: string,
      body: AdminPromptBindingCreateRequest,
      params: { idempotencyKey: string },
    ): Promise<AdminPromptBindingItem>;
    update(bindingId: string, body: AdminPromptBindingUpdateRequest): Promise<AdminPromptBindingItem>;
  };
}

export interface PageInfo {
  mode: 'offset' | 'cursor';
  page?: number;
  pageSize?: number;
  totalItems?: string;
  totalPages?: number;
  nextCursor?: string | null;
  hasMore?: boolean;
}

export interface AdminPromptListPage {
  items: AdminPromptItem[];
  pageInfo: PageInfo;
}

export interface AdminPromptVersionListPage {
  items: AdminPromptVersionItem[];
  pageInfo: PageInfo;
}

export interface AdminPromptBindingListPage {
  items: AdminPromptBindingItem[];
  pageInfo: PageInfo;
}

export interface AdminPromptRenderResult {
  rendered: string;
}

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

export interface AdminPromptVersionItem {
  id: string;
  uuid: string;
  tenantId: string;
  organizationId: string;
  promptId: string;
  versionNo: string;
  title: string;
  content: string;
  lifecycleStatus: string;
  reviewStatus: string;
  checksumHash: string;
  variableSchema: Record<string, unknown>;
  outputSchema: Record<string, unknown>;
  modelConstraints: Record<string, unknown>;
  safetyPolicy: Record<string, unknown>;
  examplesJson: Record<string, unknown>[];
  createdBy: string;
  publishedAt?: string | null;
  reviewComment?: string | null;
  createdAt: string;
  updatedAt: string;
}

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
  policyJson: Record<string, unknown>;
  snapshotJson: Record<string, unknown>;
  createdAt: string;
  updatedAt: string;
}

export interface AdminPromptCreateRequest {
  promptKey: string;
  name: string;
  description?: string;
  categoryId?: string;
  promptType?: string;
  visibility?: string;
  tags?: string[];
}

export interface AdminPromptVersionCreateRequest {
  versionNo: string;
  title: string;
  content: string;
  variableSchema?: unknown;
  outputSchema?: unknown;
  modelConstraints?: unknown;
  safetyPolicy?: unknown;
  examplesJson?: unknown[];
}

export interface AdminPromptRenderRequest {
  variables?: Record<string, unknown>;
}

export interface AdminPromptBindingCreateRequest {
  promptVersionId?: string;
  ownerType: string;
  ownerId: number;
  bindingRole: string;
  priority?: number;
  enabled?: boolean;
  policyJson?: Record<string, unknown>;
}

export interface AdminPromptBindingUpdateRequest {
  promptVersionId?: unknown;
  ownerType?: string;
  ownerId?: number;
  bindingRole?: string;
  priority?: number;
  enabled?: boolean;
  policyJson?: Record<string, unknown>;
}

export class PromptsBackendFacade {
  constructor(private readonly deps: PromptsBackendSdkDependencies) {}

  listPromptDefinitions(params?: Parameters<PromptsBackendGeneratedClient['definitions']['list']>[0]) {
    return this.deps.prompts.definitions.list(params);
  }

  createPromptDefinition(
    body: AdminPromptCreateRequest,
    idempotencyKey: string,
  ) {
    return this.deps.prompts.definitions.create(body, { idempotencyKey });
  }

  listPromptVersions(promptId: string) {
    return this.deps.prompts.versions.list(promptId);
  }

  createPromptVersion(
    promptId: string,
    body: AdminPromptVersionCreateRequest,
    idempotencyKey: string,
  ) {
    return this.deps.prompts.versions.create(promptId, body, { idempotencyKey });
  }

  publishPromptVersion(versionId: string) {
    return this.deps.prompts.versions.publish(versionId);
  }

  renderPromptVersion(versionId: string, body: AdminPromptRenderRequest) {
    return this.deps.prompts.versionRenders.create(versionId, body);
  }

  listPromptBindings(promptId: string) {
    return this.deps.prompts.definitionBindings.list(promptId);
  }

  createPromptBinding(
    promptId: string,
    body: AdminPromptBindingCreateRequest,
    idempotencyKey: string,
  ) {
    return this.deps.prompts.definitionBindings.create(promptId, body, { idempotencyKey });
  }

  updatePromptBinding(bindingId: string, body: AdminPromptBindingUpdateRequest) {
    return this.deps.prompts.definitionBindings.update(bindingId, body);
  }
}
