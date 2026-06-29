export interface PromptsOpenSdkDependencies {
  prompts: PromptsOpenGeneratedClient;
}

export interface PromptsOpenGeneratedClient {
  catalog: {
    list(): Promise<PromptCatalogPage>;
  };
}

export interface PromptCatalogEntry {
  key: string;
  name: string;
  description?: string;
}

export interface PromptCatalogPage {
  items: PromptCatalogEntry[];
  pageInfo: PageInfo;
}

export interface PageInfo {
  mode: 'offset' | 'cursor';
  page?: number;
  pageSize?: number;
  totalItems?: string;
  totalPages?: number;
  nextCursor?: string | null;
  hasMore?: boolean;
}

export class PromptsOpenFacade {
  constructor(private readonly deps: PromptsOpenSdkDependencies) {}

  listCatalog() {
    return this.deps.prompts.catalog.list();
  }
}
