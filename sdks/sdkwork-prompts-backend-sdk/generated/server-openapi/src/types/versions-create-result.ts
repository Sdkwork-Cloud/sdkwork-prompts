import type { AdminPromptVersionItem } from './admin-prompt-version-item';

export interface VersionsCreateResult {
  code: string;
  msg?: string;
  data?: Record<string, unknown>;
}
