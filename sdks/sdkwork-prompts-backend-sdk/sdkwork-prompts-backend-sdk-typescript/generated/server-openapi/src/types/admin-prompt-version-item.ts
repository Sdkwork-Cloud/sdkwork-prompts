export interface AdminPromptVersionItem {
  id: string;
  uuid: string;
  tenantId: string;
  organizationId: string;
  promptId: string;
  versionNo: string;
  title: string;
  content: string;
  lifecycleStatus: string;
  reviewStatus: string;
  checksumHash?: string;
  variableSchema?: Record<string, unknown>;
  outputSchema?: Record<string, unknown>;
  modelConstraints?: Record<string, unknown>;
  safetyPolicy?: Record<string, unknown>;
  examplesJson?: Record<string, unknown>[];
  createdBy?: string;
  publishedAt?: string;
  reviewComment?: string | null;
  createdAt: string;
  updatedAt: string;
}
