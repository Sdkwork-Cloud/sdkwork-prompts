import type { AdminPromptItem } from './admin-prompt-item';

export interface PromptsAdminDefinitionsCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
