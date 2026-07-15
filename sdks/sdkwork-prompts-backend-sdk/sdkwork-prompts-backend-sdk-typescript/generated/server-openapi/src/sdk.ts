import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkBackendConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

import { PromptsAdminApi, createPromptsAdminApi } from './api/prompts-admin';

export class SdkworkPromptsBackendClient {
  private httpClient: HttpClient;

  public readonly promptsAdmin: PromptsAdminApi;

  constructor(config: SdkworkBackendConfig) {
    this.httpClient = createHttpClient(config);
    this.promptsAdmin = createPromptsAdminApi(this.httpClient);
  }
  setAuthToken(token: string): this {
    this.httpClient.setAuthToken(token);
    return this;
  }

  setAccessToken(token: string): this {
    this.httpClient.setAccessToken(token);
    return this;
  }

  setTokenManager(manager: AuthTokenManager): this {
    this.httpClient.setTokenManager(manager);
    return this;
  }

  get http(): HttpClient {
    return this.httpClient;
  }
}

export function createClient(config: SdkworkBackendConfig): SdkworkPromptsBackendClient {
  return new SdkworkPromptsBackendClient(config);
}

export default SdkworkPromptsBackendClient;
