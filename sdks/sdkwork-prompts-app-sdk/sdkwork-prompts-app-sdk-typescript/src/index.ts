import {
  createClient as createGeneratedAppClient,
  SdkworkPromptsAppClient,
} from '../../generated/server-openapi/src/index';
import type { SdkworkAppConfig } from '../../generated/server-openapi/src/types/common';

export { SdkworkPromptsAppClient, createGeneratedAppClient };
export type { SdkworkAppConfig };
export * from '../../generated/server-openapi/src/types';
export * from '../../generated/server-openapi/src/api';
export * from '../../generated/server-openapi/src/http';
export * from '../../generated/server-openapi/src/auth';

export function createClient(config: SdkworkAppConfig): SdkworkPromptsAppClient {
  return createGeneratedAppClient(config);
}
