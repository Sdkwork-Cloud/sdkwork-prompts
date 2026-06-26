import type { AdminPromptBindingItem } from './admin-prompt-binding-item';

export interface DefinitionBindingsUpdateResult {
  code: string;
  msg?: string;
  data?: Record<string, unknown>;
}
