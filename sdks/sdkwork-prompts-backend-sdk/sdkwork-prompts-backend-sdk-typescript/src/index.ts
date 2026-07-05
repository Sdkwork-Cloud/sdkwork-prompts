import {
  createClient as createGeneratedPromptsBackendClient,
  SdkworkPromptsBackendClient,
} from '../../generated/server-openapi/src/index';
import type { SdkworkBackendConfig } from '../../generated/server-openapi/src/types/common';

export { SdkworkPromptsBackendClient, createGeneratedPromptsBackendClient };
export type { SdkworkBackendConfig };
export * from '../../generated/server-openapi/src/types';
export * from '../../generated/server-openapi/src/api';
export * from '../../generated/server-openapi/src/http';
export * from '../../generated/server-openapi/src/auth';

export function createPromptsBackendClient(
  config: SdkworkBackendConfig,
): SdkworkPromptsBackendClient {
  return createGeneratedPromptsBackendClient(config);
}

export function createClient(config: SdkworkBackendConfig): SdkworkPromptsBackendClient {
  return createPromptsBackendClient(config);
}
