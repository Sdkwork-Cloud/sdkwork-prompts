import type { AdminPromptItem } from './admin-prompt-item';

export interface DefinitionsCreateResult {
  code: string;
  msg?: string;
  data?: Record<string, unknown>;
}
