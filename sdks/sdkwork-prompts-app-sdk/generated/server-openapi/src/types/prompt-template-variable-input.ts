export interface PromptTemplateVariableInput {
  name: string;
  var_type?: string;
  required?: boolean;
  default_value?: string;
  description?: string;
}
