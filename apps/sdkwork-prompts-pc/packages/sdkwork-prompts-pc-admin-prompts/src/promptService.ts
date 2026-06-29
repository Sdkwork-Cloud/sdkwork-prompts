import type {
  AdminPromptBindingCreateRequest,
  AdminPromptBindingItem,
  AdminPromptBindingUpdateRequest,
  AdminPromptCreateRequest,
  AdminPromptItem,
  AdminPromptRenderRequest,
  AdminPromptVersionCreateRequest,
  AdminPromptVersionItem,
} from "@sdkwork/prompts-backend-sdk";
import {
  createIdempotencyParams,
  getPromptsBackendSdkClient,
  readUnwrappedApiData,
  readUnwrappedPageItems,
  readUnwrappedResourceItem,
  requiredSafePathSegment,
} from "@sdkwork/prompts-pc-commons/runtime";

type BackendPrompts = ReturnType<typeof getPromptsBackendSdkClient>["prompts"];
type ListParams<TList> = TList extends (params?: infer TParams) => unknown ? TParams : never;

export type AdminPromptListParams = ListParams<BackendPrompts["definitions"]["list"]>;
export type AdminPromptCreateInput = AdminPromptCreateRequest;
export type AdminPromptBindingCreateInput = AdminPromptBindingCreateRequest;
export type AdminPromptBindingUpdateInput = AdminPromptBindingUpdateRequest;
export type AdminPromptVersionCreateInput = AdminPromptVersionCreateRequest;
export type AdminPromptRenderInput = AdminPromptRenderRequest;

export const DEFAULT_PROMPT_PAGE_PARAMS = {
  page: 1,
  pageSize: 100,
} as const;

export async function listPrompts(params?: AdminPromptListParams): Promise<AdminPromptItem[]> {
  const result = await getPromptsBackendSdkClient().prompts.definitions.list(
    params ?? DEFAULT_PROMPT_PAGE_PARAMS,
  );
  return readUnwrappedPageItems<AdminPromptItem>(result);
}

export async function createPrompt(input: AdminPromptCreateInput): Promise<AdminPromptItem> {
  const result = await getPromptsBackendSdkClient().prompts.definitions.create(
    input,
    createIdempotencyParams("prompt-create"),
  );
  return readUnwrappedResourceItem<AdminPromptItem>(result);
}

export async function listPromptVersions(promptId: string): Promise<AdminPromptVersionItem[]> {
  const result = await getPromptsBackendSdkClient().prompts.versions.list(
    requiredSafePathSegment(promptId, "promptId"),
  );
  return readUnwrappedPageItems<AdminPromptVersionItem>(result);
}

export async function createPromptVersion(
  promptId: string,
  input: AdminPromptVersionCreateInput,
): Promise<AdminPromptVersionItem> {
  const result = await getPromptsBackendSdkClient().prompts.versions.create(
    requiredSafePathSegment(promptId, "promptId"),
    input,
    createIdempotencyParams("prompt-version-create"),
  );
  return readUnwrappedResourceItem<AdminPromptVersionItem>(result);
}

export async function publishPromptVersion(versionId: string): Promise<AdminPromptVersionItem> {
  const result = await getPromptsBackendSdkClient().prompts.versions.publish(
    requiredSafePathSegment(versionId, "versionId"),
  );
  return readUnwrappedResourceItem<AdminPromptVersionItem>(result);
}

export async function renderPromptVersion(
  versionId: string,
  input: AdminPromptRenderInput,
): Promise<{ rendered: string }> {
  const result = await getPromptsBackendSdkClient().prompts.versionRenders.create(
    requiredSafePathSegment(versionId, "versionId"),
    input,
  );
  return readUnwrappedApiData<{ rendered: string }>(result);
}

export async function listPromptBindings(promptId: string): Promise<AdminPromptBindingItem[]> {
  const result = await getPromptsBackendSdkClient().prompts.definitionBindings.list(
    requiredSafePathSegment(promptId, "promptId"),
  );
  return readUnwrappedPageItems<AdminPromptBindingItem>(result);
}

export async function createPromptBinding(
  promptId: string,
  input: AdminPromptBindingCreateInput,
): Promise<AdminPromptBindingItem> {
  const result = await getPromptsBackendSdkClient().prompts.definitionBindings.create(
    requiredSafePathSegment(promptId, "promptId"),
    input,
    createIdempotencyParams("prompt-binding-create"),
  );
  return readUnwrappedResourceItem<AdminPromptBindingItem>(result);
}

export async function updatePromptBinding(
  bindingId: string,
  input: AdminPromptBindingUpdateInput,
): Promise<AdminPromptBindingItem> {
  const result = await getPromptsBackendSdkClient().prompts.definitionBindings.update(
    requiredSafePathSegment(bindingId, "bindingId"),
    input,
  );
  return readUnwrappedResourceItem<AdminPromptBindingItem>(result);
}
