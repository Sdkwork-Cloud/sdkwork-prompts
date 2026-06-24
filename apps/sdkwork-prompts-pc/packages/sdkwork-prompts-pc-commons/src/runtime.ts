import {
  SUCCESS_CODES,
  type AuthTokenManager,
  createClient,
  type SdkworkBackendClient,
} from "@sdkwork/prompts-backend-sdk";

let client: SdkworkBackendClient | null = null;

export function resolvePromptsApiBaseUrl(): string {
  const fromEnv = import.meta.env.VITE_SDKWORK_PROMPTS_API_BASE_URL as string | undefined;
  if (fromEnv && fromEnv.trim().length > 0) {
    return fromEnv.replace(/\/$/, "");
  }
  return "http://localhost:8080";
}

export function getPromptsBackendSdkClient(): SdkworkBackendClient {
  if (!client) {
    client = createClient({
      baseUrl: resolvePromptsApiBaseUrl(),
      tenantId: (import.meta.env.VITE_SDKWORK_TENANT_ID as string | undefined) ?? "100001",
      organizationId:
        (import.meta.env.VITE_SDKWORK_ORGANIZATION_ID as string | undefined) ?? "0",
    });
  }
  return client;
}

export function setPromptsBackendTokenManager(manager: AuthTokenManager): void {
  getPromptsBackendSdkClient().setTokenManager(manager);
}

export function createIdempotencyParams(scope: string): { idempotencyKey: string } {
  return { idempotencyKey: `${scope}-${crypto.randomUUID()}` };
}

export function ensureSdkworkApiSuccess<T extends { code?: string; msg?: string }>(
  result: T,
  message: string,
): T {
  if (result.code && !SUCCESS_CODES.has(result.code)) {
    throw new Error(`${message}: ${result.msg ?? result.code}`);
  }
  return result;
}

export function requiredSafePathSegment(value: string, field: string): string {
  if (!value || /[\\/?#]/.test(value)) {
    throw new Error(`${field} must be a safe path segment`);
  }
  return value;
}
