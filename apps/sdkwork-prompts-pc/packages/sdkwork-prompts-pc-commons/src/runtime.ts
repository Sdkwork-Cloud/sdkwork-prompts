import type {
  AdminPromptBindingCreateRequest,
  AdminPromptBindingUpdateRequest,
  AdminPromptCreateRequest,
  AdminPromptRenderRequest,
  AdminPromptVersionCreateRequest,
  AuthTokenManager,
  SdkworkBackendConfig,
  SdkworkPromptsBackendClient,
} from "@sdkwork/prompts-backend-sdk";
import { createClient } from "@sdkwork/prompts-backend-sdk";
import { uuid } from "@sdkwork/utils/id";

export {
  ensureSdkworkApiSuccess,
  readApiData,
  readApiItem,
  readApiItems,
  readUnwrappedApiData,
  readUnwrappedPageItems,
  readUnwrappedResourceItem,
} from "./api-result";

export interface PromptsDefinitionsListParams {
  page?: number | string;
  pageSize?: number | string;
  q?: string;
  promptType?: string;
  visibility?: string;
  status?: string;
  categoryId?: string;
}

export interface PromptsBackendDefinitionsApi {
  list(params?: PromptsDefinitionsListParams): Promise<unknown>;
  create(
    body: AdminPromptCreateRequest,
    _params?: { idempotencyKey: string },
  ): Promise<unknown>;
}

export interface PromptsBackendVersionsApi {
  list(promptId: string): Promise<unknown>;
  create(
    promptId: string,
    body: AdminPromptVersionCreateRequest,
    _params?: { idempotencyKey: string },
  ): Promise<unknown>;
  publish(versionId: string): Promise<unknown>;
}

export interface PromptsBackendVersionRendersApi {
  create(versionId: string, body: AdminPromptRenderRequest): Promise<unknown>;
}

export interface PromptsBackendDefinitionBindingsApi {
  list(promptId: string): Promise<unknown>;
  create(
    promptId: string,
    body: AdminPromptBindingCreateRequest,
    _params?: { idempotencyKey: string },
  ): Promise<unknown>;
  update(bindingId: string, body: AdminPromptBindingUpdateRequest): Promise<unknown>;
}

export interface PromptsBackendSdkClient {
  prompts: {
    definitions: PromptsBackendDefinitionsApi;
    versions: PromptsBackendVersionsApi;
    versionRenders: PromptsBackendVersionRendersApi;
    definitionBindings: PromptsBackendDefinitionBindingsApi;
  };
  setTokenManager(manager: AuthTokenManager): void;
}

let client: PromptsBackendSdkClient | null = null;

export function resolvePromptsApiBaseUrl(): string {
  const fromEnv = import.meta.env.VITE_SDKWORK_PROMPTS_API_BASE_URL as string | undefined;
  if (fromEnv && fromEnv.trim().length > 0) {
    return fromEnv.replace(/\/$/, "");
  }
  return "http://localhost:8080";
}

function normalizeListParams(params?: PromptsDefinitionsListParams) {
  if (!params) {
    return undefined;
  }
  return {
    ...params,
    page: params.page === undefined ? undefined : Number(params.page),
    pageSize: params.pageSize === undefined ? undefined : Number(params.pageSize),
  };
}

function wrapPromptsBackendClient(generated: SdkworkPromptsBackendClient): PromptsBackendSdkClient {
  const admin = generated.promptsAdmin.prompts.admin;
  return {
    prompts: {
      definitions: {
        list: (params) => admin.definitions.list(normalizeListParams(params)),
        create: (body, _params) => admin.definitions.create(body),
      },
      versions: {
        list: (promptId) => admin.versions.list(promptId),
        create: (promptId, body, _params) => admin.versions.create(promptId, body),
        publish: (versionId) => admin.versions.publish(versionId),
      },
      versionRenders: {
        create: (versionId, body) => admin.versions.render(versionId, body),
      },
      definitionBindings: {
        list: (promptId) => admin.bindings.list(promptId),
        create: (promptId, body, _params) => admin.bindings.create(promptId, body),
        update: (bindingId, body) => admin.bindings.update(bindingId, body),
      },
    },
    setTokenManager: (manager) => {
      generated.setTokenManager(manager);
    },
  };
}

export function getPromptsBackendSdkClient(): PromptsBackendSdkClient {
  if (!client) {
    const config: SdkworkBackendConfig = {
      baseUrl: resolvePromptsApiBaseUrl(),
      tenantId: (import.meta.env.VITE_SDKWORK_TENANT_ID as string | undefined) ?? "100001",
      organizationId:
        (import.meta.env.VITE_SDKWORK_ORGANIZATION_ID as string | undefined) ?? "0",
    };
    client = wrapPromptsBackendClient(createClient(config));
  }
  return client;
}

export function setPromptsBackendTokenManager(manager: AuthTokenManager): void {
  getPromptsBackendSdkClient().setTokenManager(manager);
}

export function createIdempotencyParams(scope: string): { idempotencyKey: string } {
  return { idempotencyKey: `${scope}-${uuid()}` };
}

export function requiredSafePathSegment(value: string, field: string): string {
  if (!value || /[\\/?#]/.test(value)) {
    throw new Error(`${field} must be a safe path segment`);
  }
  return value;
}
