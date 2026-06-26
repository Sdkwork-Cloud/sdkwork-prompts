import { appApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { AgentPromptTemplate, AgentPromptTemplatePage, PromptTemplate, PromptTemplateCreateRequest, PromptTemplatePage, PromptTemplateUpdateRequest, PromptTemplateVersion, PromptTemplateVersionCreateRequest, PromptTemplateVersionPage } from '../types';


export interface PromptsAgentTemplatesListParams {
  limit?: number;
}

export class PromptsAgentTemplatesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List agent prompt templates. */
  async list(params?: PromptsAgentTemplatesListParams): Promise<AgentPromptTemplatePage> {
    const query = buildQueryString([
      { name: 'limit', value: params?.limit, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<AgentPromptTemplatePage>(appendQueryString(appApiPath(`/prompts/agent_templates`), query));
  }

/** Get agent prompt template. */
  async get(templateId: string): Promise<AgentPromptTemplate> {
    return this.client.get<AgentPromptTemplate>(appApiPath(`/prompts/agent_templates/${serializePathParameter(templateId, { name: 'templateId', style: 'simple', explode: false })}`));
  }
}

export class PromptsTemplateVersionsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List template versions. */
  async list(templateId: string): Promise<PromptTemplateVersionPage> {
    return this.client.get<PromptTemplateVersionPage>(appApiPath(`/prompts/templates/${serializePathParameter(templateId, { name: 'templateId', style: 'simple', explode: false })}/versions`));
  }

/** Create template version. */
  async create(templateId: string, body: PromptTemplateVersionCreateRequest): Promise<PromptTemplateVersion> {
    return this.client.post<PromptTemplateVersion>(appApiPath(`/prompts/templates/${serializePathParameter(templateId, { name: 'templateId', style: 'simple', explode: false })}/versions`), body, undefined, undefined, 'application/json');
  }
}

export interface PromptsTemplatesListParams {
  cursor?: string;
  limit?: number;
  status?: 'draft' | 'active' | 'archived';
}

export class PromptsTemplatesApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** List prompt templates. */
  async list(params?: PromptsTemplatesListParams): Promise<PromptTemplatePage> {
    const query = buildQueryString([
      { name: 'cursor', value: params?.cursor, style: 'form', explode: true, allowReserved: false },
      { name: 'limit', value: params?.limit, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<PromptTemplatePage>(appendQueryString(appApiPath(`/prompts/templates`), query));
  }

/** Create prompt template. */
  async create(body: PromptTemplateCreateRequest): Promise<PromptTemplate> {
    return this.client.post<PromptTemplate>(appApiPath(`/prompts/templates`), body, undefined, undefined, 'application/json');
  }

/** Get prompt template. */
  async get(templateId: string): Promise<PromptTemplate> {
    return this.client.get<PromptTemplate>(appApiPath(`/prompts/templates/${serializePathParameter(templateId, { name: 'templateId', style: 'simple', explode: false })}`));
  }

/** Update prompt template metadata. */
  async update(templateId: string, body: PromptTemplateUpdateRequest): Promise<PromptTemplate> {
    return this.client.patch<PromptTemplate>(appApiPath(`/prompts/templates/${serializePathParameter(templateId, { name: 'templateId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export class PromptsApi {
  private client: HttpClient;
  public readonly templates: PromptsTemplatesApi;
  public readonly templateVersions: PromptsTemplateVersionsApi;
  public readonly agentTemplates: PromptsAgentTemplatesApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.templates = new PromptsTemplatesApi(client);
    this.templateVersions = new PromptsTemplateVersionsApi(client);
    this.agentTemplates = new PromptsAgentTemplatesApi(client);
  }

}

export function createPromptsApi(client: HttpClient): PromptsApi {
  return new PromptsApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}

interface PathParameterSpec {
  name: string;
  style: string;
  explode: boolean;
}

function serializePathParameter(value: unknown, spec: PathParameterSpec): string {
  if (value === undefined || value === null) {
    return '';
  }

  const style = spec.style || 'simple';
  if (Array.isArray(value)) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (typeof value === 'object') {
    return serializePathObject(spec.name, value as Record<string, unknown>, style, spec.explode);
  }
  return pathPrefix(spec.name, style, false) + encodePathValue(serializePathPrimitive(value));
}

function serializePathArray(name: string, values: unknown[], style: string, explode: boolean): string {
  const serialized = values
    .filter((item) => item !== undefined && item !== null)
    .map((item) => encodePathValue(serializePathPrimitive(item)));
  if (serialized.length === 0) {
    return pathPrefix(name, style, false);
  }
  if (style === 'matrix') {
    return explode
      ? serialized.map((item) => `;${name}=${item}`).join('')
      : `;${name}=${serialized.join(',')}`;
  }
  return pathPrefix(name, style, false) + serialized.join(explode ? '.' : ',');
}

function serializePathObject(name: string, value: Record<string, unknown>, style: string, explode: boolean): string {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return pathPrefix(name, style, true);
  }
  if (style === 'matrix') {
    return explode
      ? entries.map(([key, entryValue]) => `;${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join('')
      : `;${name}=${entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',')}`;
  }
  const serialized = explode
    ? entries.map(([key, entryValue]) => `${encodePathValue(key)}=${encodePathValue(serializePathPrimitive(entryValue))}`).join(style === 'label' ? '.' : ',')
    : entries.flatMap(([key, entryValue]) => [encodePathValue(key), encodePathValue(serializePathPrimitive(entryValue))]).join(',');
  return pathPrefix(name, style, true) + serialized;
}

function pathPrefix(name: string, style: string, _objectValue: boolean): string {
  if (style === 'label') return '.';
  if (style === 'matrix') return `;${name}`;
  return '';
}

function encodePathValue(value: string): string {
  return encodeURIComponent(value);
}

function serializePathPrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}
interface QueryParameterSpec {
  name: string;
  value: unknown;
  style: string;
  explode: boolean;
  allowReserved: boolean;
  contentType?: string;
}

function buildQueryString(parameters: QueryParameterSpec[]): string {
  const pairs: string[] = [];
  for (const parameter of parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

function appendSerializedParameter(pairs: string[], parameter: QueryParameterSpec): void {
  if (parameter.value === undefined || parameter.value === null) {
    return;
  }

  if (parameter.contentType) {
    pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(JSON.stringify(parameter.value), parameter.allowReserved)}`);
    return;
  }

  const style = parameter.style || 'form';
  if (style === 'deepObject') {
    appendDeepObjectParameter(pairs, parameter.name, parameter.value, parameter.allowReserved);
    return;
  }

  if (Array.isArray(parameter.value)) {
    appendArrayParameter(pairs, parameter.name, parameter.value, style, parameter.explode, parameter.allowReserved);
    return;
  }

  if (typeof parameter.value === 'object') {
    appendObjectParameter(pairs, parameter.name, parameter.value as Record<string, unknown>, style, parameter.explode, parameter.allowReserved);
    return;
  }

  pairs.push(`${encodeQueryComponent(parameter.name)}=${encodeQueryValue(serializePrimitive(parameter.value), parameter.allowReserved)}`);
}

function appendArrayParameter(
  pairs: string[],
  name: string,
  value: unknown[],
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const values = value
    .filter((item) => item !== undefined && item !== null)
    .map((item) => serializePrimitive(item));
  if (values.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const item of values) {
      pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(item, allowReserved)}`);
    }
    return;
  }

  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(values.join(','), allowReserved)}`);
}

function appendObjectParameter(
  pairs: string[],
  name: string,
  value: Record<string, unknown>,
  style: string,
  explode: boolean,
  allowReserved: boolean,
): void {
  const entries = Object.entries(value).filter(([, entryValue]) => entryValue !== undefined && entryValue !== null);
  if (entries.length === 0) {
    return;
  }

  if (style === 'form' && explode) {
    for (const [key, entryValue] of entries) {
      pairs.push(`${encodeQueryComponent(key)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
    }
    return;
  }

  const serialized = entries.flatMap(([key, entryValue]) => [key, serializePrimitive(entryValue)]).join(',');
  pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serialized, allowReserved)}`);
}

function appendDeepObjectParameter(
  pairs: string[],
  name: string,
  value: unknown,
  allowReserved: boolean,
): void {
  if (!value || typeof value !== 'object' || Array.isArray(value)) {
    pairs.push(`${encodeQueryComponent(name)}=${encodeQueryValue(serializePrimitive(value), allowReserved)}`);
    return;
  }

  for (const [key, entryValue] of Object.entries(value as Record<string, unknown>)) {
    if (entryValue === undefined || entryValue === null) {
      continue;
    }
    pairs.push(`${encodeQueryComponent(`${name}[${key}]`)}=${encodeQueryValue(serializePrimitive(entryValue), allowReserved)}`);
  }
}

function serializePrimitive(value: unknown): string {
  if (value instanceof Date) {
    return value.toISOString();
  }
  if (typeof value === 'object') {
    return JSON.stringify(value);
  }
  return String(value);
}

function encodeQueryComponent(value: string): string {
  return encodeURIComponent(value);
}

function encodeQueryValue(value: string, allowReserved: boolean): string {
  const encoded = encodeURIComponent(value);
  if (!allowReserved) {
    return encoded;
  }
  return encoded.replace(/%3A/gi, ':')
    .replace(/%2F/gi, '/')
    .replace(/%3F/gi, '?')
    .replace(/%23/gi, '#')
    .replace(/%5B/gi, '[')
    .replace(/%5D/gi, ']')
    .replace(/%40/gi, '@')
    .replace(/%21/gi, '!')
    .replace(/%24/gi, '$')
    .replace(/%26/gi, '&')
    .replace(/%27/gi, "'")
    .replace(/%28/gi, '(')
    .replace(/%29/gi, ')')
    .replace(/%2A/gi, '*')
    .replace(/%2B/gi, '+')
    .replace(/%2C/gi, ',')
    .replace(/%3B/gi, ';')
    .replace(/%3D/gi, '=');
}
