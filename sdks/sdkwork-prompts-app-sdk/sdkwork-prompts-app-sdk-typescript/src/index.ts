import {
  createClient as createGeneratedAppClient,
  SdkworkPromptsAppClient,
  type SdkworkAppConfig,
} from 'sdkwork-prompts-app-sdk-generated-typescript';

export { SdkworkPromptsAppClient, createGeneratedAppClient };
export type { SdkworkAppConfig };
export * from 'sdkwork-prompts-app-sdk-generated-typescript';

export function createClient(config: SdkworkAppConfig): SdkworkPromptsAppClient {
  return createGeneratedAppClient(config);
}
