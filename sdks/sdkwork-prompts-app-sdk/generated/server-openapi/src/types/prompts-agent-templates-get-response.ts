import type { AgentPromptTemplate } from './agent-prompt-template';

export interface PromptsAgentTemplatesGetResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
