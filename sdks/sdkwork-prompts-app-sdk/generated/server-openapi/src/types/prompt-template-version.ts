import type { PromptTemplateVariable } from './prompt-template-variable';

export interface PromptTemplateVersion {
  id: string;
  template_id: string;
  version_label: string;
  content: string;
  model_hint?: string | null;
  status: 'draft' | 'active' | 'archived';
  variables?: PromptTemplateVariable[];
}
