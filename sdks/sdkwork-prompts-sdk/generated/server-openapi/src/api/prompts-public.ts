import { customApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { PromptCatalogPage } from '../types';


export class PromptsPublicPromptsCatalogApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(): Promise<PromptCatalogPage> {
    return this.client.get<PromptCatalogPage>(customApiPath(`/prompts/catalog`));
  }
}

export class PromptsPublicPromptsApi {
  private client: HttpClient;
  public readonly catalog: PromptsPublicPromptsCatalogApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.catalog = new PromptsPublicPromptsCatalogApi(client);
  }

}

export class PromptsPublicApi {
  private client: HttpClient;
  public readonly prompts: PromptsPublicPromptsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.prompts = new PromptsPublicPromptsApi(client);
  }

}

export function createPromptsPublicApi(client: HttpClient): PromptsPublicApi {
  return new PromptsPublicApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
