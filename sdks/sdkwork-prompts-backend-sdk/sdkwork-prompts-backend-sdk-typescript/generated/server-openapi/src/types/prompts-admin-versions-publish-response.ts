import type { AdminPromptVersionItem } from './admin-prompt-version-item';

export interface PromptsAdminVersionsPublishResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
