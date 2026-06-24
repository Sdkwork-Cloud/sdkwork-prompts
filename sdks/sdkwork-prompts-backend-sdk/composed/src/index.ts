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
    }): Promise<DefinitionsListResult>;
    create(
      body: AdminPromptCreateRequest,
      params: { idempotencyKey: string },
    ): Promise<DefinitionsCreateResult>;
  };
  versions: {
    list(promptId: string): Promise<VersionsListResult>;
    create(
      promptId: string,
      body: AdminPromptVersionCreateRequest,
      params: { idempotencyKey: string },
    ): Promise<VersionsCreateResult>;
    publish(versionId: string): Promise<VersionsPublishResult>;
  };
  versionRenders: {
    create(versionId: string, body: AdminPromptRenderRequest): Promise<VersionRendersCreateResult>;
  };
  definitionBindings: {
    list(promptId: string): Promise<DefinitionBindingsListResult>;
    create(
      promptId: string,
      body: AdminPromptBindingCreateRequest,
      params: { idempotencyKey: string },
    ): Promise<DefinitionBindingsCreateResult>;
    update(bindingId: string, body: AdminPromptBindingUpdateRequest): Promise<DefinitionBindingsUpdateResult>;
  };
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

export interface PlusApiResult {
  code: string;
  msg?: string;
  data?: unknown;
}

export type DefinitionsListResult = PlusApiResult;
export type DefinitionsCreateResult = PlusApiResult;
export type VersionsListResult = PlusApiResult;
export type VersionsCreateResult = PlusApiResult;
export type VersionsPublishResult = PlusApiResult;
export type VersionRendersCreateResult = PlusApiResult;
export type DefinitionBindingsListResult = PlusApiResult;
export type DefinitionBindingsCreateResult = PlusApiResult;
export type DefinitionBindingsUpdateResult = PlusApiResult;

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
