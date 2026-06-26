import type { PromptTemplate } from './prompt-template';

export interface PromptTemplatePage {
  items: PromptTemplate[];
  next_cursor?: string | null;
}
