import type { PromptTemplate } from './prompt-template';

export interface PromptsTemplatesCreateResponse201 {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
