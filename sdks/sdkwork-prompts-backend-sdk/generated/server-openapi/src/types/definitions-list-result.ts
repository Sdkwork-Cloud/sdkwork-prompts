import type { AdminPromptItem } from './admin-prompt-item';

export interface DefinitionsListResult {
  code: string;
  msg?: string;
  data?: Record<string, unknown>;
}
