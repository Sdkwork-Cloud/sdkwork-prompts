import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { AdminPromptBindingCreateRequest, AdminPromptBindingUpdateRequest, AdminPromptCreateRequest, AdminPromptRenderRequest, AdminPromptVersionCreateRequest, DefinitionBindingsCreateResult, DefinitionBindingsListResult, DefinitionBindingsUpdateResult, DefinitionsCreateResult, DefinitionsListResult, VersionRendersCreateResult, VersionsCreateResult, VersionsListResult, VersionsPublishResult } from '../types';


export class PromptsAdminPromptsAdminBindingsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(promptId: string): Promise<DefinitionBindingsListResult> {
    return this.client.get<DefinitionBindingsListResult>(backendApiPath(`/prompts/${serializePathParameter(promptId, { name: 'promptId', style: 'simple', explode: false })}/bindings`));
  }

async create(promptId: string, body: AdminPromptBindingCreateRequest): Promise<DefinitionBindingsCreateResult> {
    return this.client.post<DefinitionBindingsCreateResult>(backendApiPath(`/prompts/${serializePathParameter(promptId, { name: 'promptId', style: 'simple', explode: false })}/bindings`), body, undefined, undefined, 'application/json');
  }

async update(bindingId: string, body: AdminPromptBindingUpdateRequest): Promise<DefinitionBindingsUpdateResult> {
    return this.client.put<DefinitionBindingsUpdateResult>(backendApiPath(`/prompts/bindings/${serializePathParameter(bindingId, { name: 'bindingId', style: 'simple', explode: false })}`), body, undefined, undefined, 'application/json');
  }
}

export class PromptsAdminPromptsAdminVersionsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(promptId: string): Promise<VersionsListResult> {
    return this.client.get<VersionsListResult>(backendApiPath(`/prompts/${serializePathParameter(promptId, { name: 'promptId', style: 'simple', explode: false })}/versions`));
  }

async create(promptId: string, body: AdminPromptVersionCreateRequest): Promise<VersionsCreateResult> {
    return this.client.post<VersionsCreateResult>(backendApiPath(`/prompts/${serializePathParameter(promptId, { name: 'promptId', style: 'simple', explode: false })}/versions`), body, undefined, undefined, 'application/json');
  }

async publish(versionId: string): Promise<VersionsPublishResult> {
    return this.client.post<VersionsPublishResult>(backendApiPath(`/prompts/versions/${serializePathParameter(versionId, { name: 'versionId', style: 'simple', explode: false })}/publish`));
  }

async render(versionId: string, body?: AdminPromptRenderRequest): Promise<VersionRendersCreateResult> {
    return this.client.post<VersionRendersCreateResult>(backendApiPath(`/prompts/versions/${serializePathParameter(versionId, { name: 'versionId', style: 'simple', explode: false })}/render`), body, undefined, undefined, 'application/json');
  }
}

export interface PromptsAdminPromptsAdminDefinitionsListParams {
  page?: number;
  pageSize?: number;
  q?: string;
  promptType?: string;
  visibility?: string;
  status?: string;
  categoryId?: string;
}

export class PromptsAdminPromptsAdminDefinitionsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


async list(params?: PromptsAdminPromptsAdminDefinitionsListParams): Promise<DefinitionsListResult> {
    const query = buildQueryString([
      { name: 'page', value: params?.page, style: 'form', explode: true, allowReserved: false },
      { name: 'page_size', value: params?.pageSize, style: 'form', explode: true, allowReserved: false },
      { name: 'q', value: params?.q, style: 'form', explode: true, allowReserved: false },
      { name: 'prompt_type', value: params?.promptType, style: 'form', explode: true, allowReserved: false },
      { name: 'visibility', value: params?.visibility, style: 'form', explode: true, allowReserved: false },
      { name: 'status', value: params?.status, style: 'form', explode: true, allowReserved: false },
      { name: 'category_id', value: params?.categoryId, style: 'form', explode: true, allowReserved: false },
    ]);
    return this.client.get<DefinitionsListResult>(appendQueryString(backendApiPath(`/prompts`), query));
  }

async create(body: AdminPromptCreateRequest): Promise<DefinitionsCreateResult> {
    return this.client.post<DefinitionsCreateResult>(backendApiPath(`/prompts`), body, undefined, undefined, 'application/json');
  }
}

export class PromptsAdminPromptsAdminApi {
  private client: HttpClient;
  public readonly definitions: PromptsAdminPromptsAdminDefinitionsApi;
  public readonly versions: PromptsAdminPromptsAdminVersionsApi;
  public readonly bindings: PromptsAdminPromptsAdminBindingsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.definitions = new PromptsAdminPromptsAdminDefinitionsApi(client);
    this.versions = new PromptsAdminPromptsAdminVersionsApi(client);
    this.bindings = new PromptsAdminPromptsAdminBindingsApi(client);
  }

}

export class PromptsAdminPromptsApi {
  private client: HttpClient;
  public readonly admin: PromptsAdminPromptsAdminApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.admin = new PromptsAdminPromptsAdminApi(client);
  }

}

export class PromptsAdminApi {
  private client: HttpClient;
  public readonly prompts: PromptsAdminPromptsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.prompts = new PromptsAdminPromptsApi(client);
  }

}

export function createPromptsAdminApi(client: HttpClient): PromptsAdminApi {
  return new PromptsAdminApi(client);
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
