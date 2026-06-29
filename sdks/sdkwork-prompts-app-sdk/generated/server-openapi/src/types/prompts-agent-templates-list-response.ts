import type { AgentPromptTemplate } from './agent-prompt-template';
import type { PageInfo } from './page-info';

export interface PromptsAgentTemplatesListResponse {
  code: 0;
  data: unknown & Record<string, unknown>;
  /** Server-owned request correlation id. */
  traceId: string;
}
