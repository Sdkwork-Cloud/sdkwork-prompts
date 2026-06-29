import type { PromptTemplate } from './prompt-template';

export interface PromptsTemplatesGetResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
