import type { PromptTemplate } from './prompt-template';

export interface PromptsTemplatesUpdateResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
