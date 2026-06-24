/** Versions list result schema exposed by Claw Router. */
export interface VersionsListResult {
  /** Business response code. */
  code: string;
  /** Data field on versions list result. */
  data?: unknown;
  /** Human-readable response message. */
  msg?: string;
}
