export interface PromptsAppSdkDependencies {
  prompts: PromptsAppGeneratedClient;
}

export interface PromptsAppGeneratedClient {
  templates: {
    list(params?: { cursor?: string; limit?: number; status?: string }): Promise<PromptTemplatePage>;
    create(body: PromptTemplateCreateRequest): Promise<PromptTemplate>;
    retrieve(templateId: string): Promise<PromptTemplate>;
    update(templateId: string, body: PromptTemplateUpdateRequest): Promise<PromptTemplate>;
    versions: {
      list(templateId: string): Promise<PromptTemplateVersionPage>;
      create(templateId: string, body: PromptTemplateVersionCreateRequest): Promise<PromptTemplateVersion>;
    };
  };
  agentTemplates: {
    list(params?: { limit?: number }): Promise<AgentPromptTemplatePage>;
    retrieve(templateId: string): Promise<AgentPromptTemplate>;
  };
}

export interface PromptTemplate {
  id: string;
  key: string;
  name: string;
  description?: string;
  status: 'draft' | 'active' | 'archived';
  tags?: string[];
  latest_version_id?: string | null;
  updated_at?: string;
}

export interface PromptTemplatePage {
  items: PromptTemplate[];
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

export interface PromptTemplateCreateRequest {
  key: string;
  name: string;
  description?: string;
  tags?: string[];
}

export interface PromptTemplateUpdateRequest {
  name?: string;
  description?: string;
  status?: 'draft' | 'active' | 'archived';
  tags?: string[];
}

export interface PromptTemplateVersion {
  id: string;
  template_id: string;
  version_label: string;
  content: string;
  model_hint?: string | null;
  status: 'draft' | 'active' | 'archived';
  variables?: PromptTemplateVariable[];
}

export interface PromptTemplateVersionPage {
  items: PromptTemplateVersion[];
  pageInfo: PageInfo;
}

export interface PromptTemplateVersionCreateRequest {
  version_label: string;
  content: string;
  model_hint?: string;
  variables?: PromptTemplateVariableInput[];
}

export interface PromptTemplateVariable {
  name: string;
  var_type: string;
  required: boolean;
  default_value?: string | null;
  description?: string | null;
}

export interface PromptTemplateVariableInput {
  name: string;
  var_type?: string;
  required?: boolean;
  default_value?: string;
  description?: string;
}

export interface AgentPromptTemplate {
  id: string;
  uuid?: string;
  promptId?: string;
  code: string;
  displayName: string;
  description?: string | null;
  promptKind: string;
  templateFormat: string;
  templateBody?: string;
  safetyProfileId?: string | null;
  status: number;
  visibility?: number;
}

export interface AgentPromptTemplatePage {
  items: AgentPromptTemplate[];
  pageInfo: PageInfo;
}

export class PromptsAppFacade {
  constructor(private readonly deps: PromptsAppSdkDependencies) {}

  listTemplates(params?: Parameters<PromptsAppGeneratedClient['templates']['list']>[0]) {
    return this.deps.prompts.templates.list(params);
  }

  createTemplate(body: PromptTemplateCreateRequest) {
    return this.deps.prompts.templates.create(body);
  }

  getTemplate(templateId: string) {
    return this.deps.prompts.templates.retrieve(templateId);
  }

  updateTemplate(templateId: string, body: PromptTemplateUpdateRequest) {
    return this.deps.prompts.templates.update(templateId, body);
  }

  listTemplateVersions(templateId: string) {
    return this.deps.prompts.templates.versions.list(templateId);
  }

  createTemplateVersion(templateId: string, body: PromptTemplateVersionCreateRequest) {
    return this.deps.prompts.templates.versions.create(templateId, body);
  }

  listAgentTemplates(params?: Parameters<PromptsAppGeneratedClient['agentTemplates']['list']>[0]) {
    return this.deps.prompts.agentTemplates.list(params);
  }

  getAgentTemplate(templateId: string) {
    return this.deps.prompts.agentTemplates.retrieve(templateId);
  }
}
