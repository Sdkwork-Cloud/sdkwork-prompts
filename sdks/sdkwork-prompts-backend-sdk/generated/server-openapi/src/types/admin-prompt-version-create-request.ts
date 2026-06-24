/** Admin prompt version create request schema exposed by Claw Router. */
export interface AdminPromptVersionCreateRequest {
  /** Content field on admin prompt version create request. */
  content: string;
  /** Examples json field on admin prompt version create request. */
  examplesJson?: Record<string, unknown>[] | Record<string, unknown>;
  /** Model constraints field on admin prompt version create request. */
  modelConstraints?: Record<string, unknown>;
  /** Output schema field on admin prompt version create request. */
  outputSchema?: Record<string, unknown>;
  /** Safety policy field on admin prompt version create request. */
  safetyPolicy?: Record<string, unknown>;
  /** Title field on admin prompt version create request. */
  title: string;
  /** Variable schema field on admin prompt version create request. */
  variableSchema?: Record<string, unknown>;
  /** Version no field on admin prompt version create request. */
  versionNo: string;
}
