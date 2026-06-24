import type {
  AdminPromptBindingCreateRequest,
  AdminPromptBindingUpdateRequest,
  AdminPromptCreateRequest,
  AdminPromptRenderRequest,
  AdminPromptVersionCreateRequest,
} from "@sdkwork/prompts-backend-sdk";
import {
  createIdempotencyParams,
  ensureSdkworkApiSuccess,
  getPromptsBackendSdkClient,
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
  page: "1",
  pageSize: "100",
} as const;

export async function listPrompts(params?: AdminPromptListParams) {
  return getPromptsBackendSdkClient().prompts.definitions.list(
    params ?? DEFAULT_PROMPT_PAGE_PARAMS,
  );
}

export async function createPrompt(input: AdminPromptCreateInput) {
  const result = await getPromptsBackendSdkClient().prompts.definitions.create(
    input,
    createIdempotencyParams("prompt-create"),
  );
  return ensureSdkworkApiSuccess(result, "Failed to create prompt");
}

export async function listPromptVersions(promptId: string) {
  return getPromptsBackendSdkClient().prompts.versions.list(
    requiredSafePathSegment(promptId, "promptId"),
  );
}

export async function createPromptVersion(
  promptId: string,
  input: AdminPromptVersionCreateInput,
) {
  const result = await getPromptsBackendSdkClient().prompts.versions.create(
    requiredSafePathSegment(promptId, "promptId"),
    input,
    createIdempotencyParams("prompt-version-create"),
  );
  return ensureSdkworkApiSuccess(result, "Failed to create prompt version");
}

export async function publishPromptVersion(versionId: string) {
  const result = await getPromptsBackendSdkClient().prompts.versions.publish(
    requiredSafePathSegment(versionId, "versionId"),
  );
  return ensureSdkworkApiSuccess(result, "Failed to publish prompt version");
}

export async function renderPromptVersion(versionId: string, input: AdminPromptRenderInput) {
  const result = await getPromptsBackendSdkClient().prompts.versionRenders.create(
    requiredSafePathSegment(versionId, "versionId"),
    input,
  );
  return ensureSdkworkApiSuccess(result, "Failed to render prompt version");
}

export async function listPromptBindings(promptId: string) {
  return getPromptsBackendSdkClient().prompts.definitionBindings.list(
    requiredSafePathSegment(promptId, "promptId"),
  );
}

export async function createPromptBinding(
  promptId: string,
  input: AdminPromptBindingCreateInput,
) {
  const result = await getPromptsBackendSdkClient().prompts.definitionBindings.create(
    requiredSafePathSegment(promptId, "promptId"),
    input,
    createIdempotencyParams("prompt-binding-create"),
  );
  return ensureSdkworkApiSuccess(result, "Failed to create prompt binding");
}

export async function updatePromptBinding(
  bindingId: string,
  input: AdminPromptBindingUpdateInput,
) {
  const result = await getPromptsBackendSdkClient().prompts.definitionBindings.update(
    requiredSafePathSegment(bindingId, "bindingId"),
    input,
  );
  return ensureSdkworkApiSuccess(result, "Failed to update prompt binding");
}
