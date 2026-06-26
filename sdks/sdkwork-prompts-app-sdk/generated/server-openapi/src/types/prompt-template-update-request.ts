export interface PromptTemplateUpdateRequest {
  name?: string;
  description?: string;
  status?: 'draft' | 'active' | 'archived';
  tags?: string[];
}
