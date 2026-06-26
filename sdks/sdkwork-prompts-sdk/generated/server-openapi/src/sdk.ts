import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkCustomConfig } from './types/common';

import { PromptsPublicApi, createPromptsPublicApi } from './api/prompts-public';

export class SdkworkPromptsOpenClient {
  private httpClient: HttpClient;

  public readonly promptsPublic: PromptsPublicApi;

  constructor(config: SdkworkCustomConfig) {
    this.httpClient = createHttpClient(config);
    this.promptsPublic = createPromptsPublicApi(this.httpClient);
  }

  setApiKey(apiKey: string): this {
    this.httpClient.setApiKey(apiKey);
    return this;
  }
  get http(): HttpClient {
    return this.httpClient;
  }
}

export function createClient(config: SdkworkCustomConfig): SdkworkPromptsOpenClient {
  return new SdkworkPromptsOpenClient(config);
}

export default SdkworkPromptsOpenClient;
