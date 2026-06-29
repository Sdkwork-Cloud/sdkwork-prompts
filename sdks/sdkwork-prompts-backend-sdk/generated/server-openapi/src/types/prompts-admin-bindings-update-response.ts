import type { AdminPromptBindingItem } from './admin-prompt-binding-item';

export interface PromptsAdminBindingsUpdateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
