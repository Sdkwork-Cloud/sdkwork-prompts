export interface AdminPromptVersionCreateRequest {
  versionNo: string;
  title: string;
  content: string;
  variableSchema?: Record<string, unknown>;
  outputSchema?: Record<string, unknown>;
  modelConstraints?: Record<string, unknown>;
  safetyPolicy?: Record<string, unknown>;
  examplesJson?: unknown[];
}
