import type { PromptTemplateVariableInput } from './prompt-template-variable-input';

export interface PromptTemplateVersionCreateRequest {
  version_label: string;
  content: string;
  model_hint?: string;
  variables?: PromptTemplateVariableInput[];
}
