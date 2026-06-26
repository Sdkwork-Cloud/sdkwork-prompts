import type { AdminPromptBindingItem } from './admin-prompt-binding-item';

export interface DefinitionBindingsCreateResult {
  code: string;
  msg?: string;
  data?: Record<string, unknown>;
}
