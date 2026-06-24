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
}

export class PromptsOpenFacade {
  constructor(private readonly deps: PromptsOpenSdkDependencies) {}

  listCatalog() {
    return this.deps.prompts.catalog.list();
  }
}
