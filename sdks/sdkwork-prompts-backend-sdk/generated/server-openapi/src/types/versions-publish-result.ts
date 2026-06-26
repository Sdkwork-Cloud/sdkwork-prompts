import type { AdminPromptVersionItem } from './admin-prompt-version-item';

export interface VersionsPublishResult {
  code: string;
  msg?: string;
  data?: Record<string, unknown>;
}
