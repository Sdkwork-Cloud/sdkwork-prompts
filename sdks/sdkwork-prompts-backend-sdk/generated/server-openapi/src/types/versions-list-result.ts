import type { AdminPromptVersionItem } from './admin-prompt-version-item';

export interface VersionsListResult {
  code: string;
  msg?: string;
  data?: Record<string, unknown>;
}
