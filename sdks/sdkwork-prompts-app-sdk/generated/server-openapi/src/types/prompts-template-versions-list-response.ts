import type { PageInfo } from './page-info';
import type { PromptTemplateVersion } from './prompt-template-version';

export interface PromptsTemplateVersionsListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
