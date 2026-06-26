export interface PromptTemplateVariable {
  name: string;
  var_type: string;
  required: boolean;
  default_value?: string | null;
  description?: string | null;
}
