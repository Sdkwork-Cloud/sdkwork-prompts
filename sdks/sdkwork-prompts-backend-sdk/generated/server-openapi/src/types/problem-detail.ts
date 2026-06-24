export interface ProblemDetail {
  code?: string;
  detail?: string;
  errors?: unknown[];
  instance?: string;
  /** Server-owned request correlation id. */
  requestId?: string;
  status: number;
  title: string;
  traceId?: string;
  type: string;
}
