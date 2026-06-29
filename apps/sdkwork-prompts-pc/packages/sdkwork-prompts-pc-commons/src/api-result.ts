import {
  isSdkWorkSuccessCode,
  unwrapSdkWorkApiResponse,
  type SdkWorkApiResponse,
  type SdkWorkPageData,
} from "@sdkwork/utils";

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function asSdkWorkEnvelope(result: unknown): SdkWorkApiResponse<unknown> | null {
  if (!isRecord(result)) {
    return null;
  }
  if (typeof result.code !== "number" || !("data" in result) || typeof result.traceId !== "string") {
    return null;
  }
  return result as SdkWorkApiResponse<unknown>;
}

export function ensureSdkworkApiSuccess(result: unknown, message: string): void {
  const envelope = asSdkWorkEnvelope(result);
  if (!envelope) {
    throw new Error(message);
  }
  if (!isSdkWorkSuccessCode(envelope.code)) {
    throw new Error(`${message}: ${envelope.code}`);
  }
}

export function readApiData<TData = unknown>(result: unknown): TData {
  const envelope = asSdkWorkEnvelope(result);
  if (!envelope) {
    throw new Error("Expected SdkWorkApiResponse envelope");
  }
  return unwrapSdkWorkApiResponse(envelope) as TData;
}

export function readApiItems<TItem = unknown>(result: unknown): TItem[] {
  const data = readApiData<SdkWorkPageData<TItem>>(result);
  return Array.isArray(data.items) ? data.items : [];
}

export function readApiItem<TItem = unknown>(result: unknown): TItem {
  const data = readApiData<{ item: TItem }>(result);
  if (!data || typeof data !== "object" || !("item" in data)) {
    throw new Error("Expected SdkWorkResourceData.item payload");
  }
  return data.item;
}

/** Reads list items from an sdkwork-v3 unwrapped page or a full SdkWorkApiResponse envelope. */
export function readUnwrappedPageItems<TItem>(result: unknown): TItem[] {
  if (result && typeof result === "object" && "items" in result) {
    const items = (result as SdkWorkPageData<TItem>).items;
    return Array.isArray(items) ? items : [];
  }
  return readApiItems<TItem>(result);
}

/** Reads a single resource from an sdkwork-v3 unwrapped item or a full SdkWorkApiResponse envelope. */
export function readUnwrappedResourceItem<TItem>(result: unknown): TItem {
  if (result && typeof result === "object" && !("code" in result) && !("traceId" in result)) {
    return result as TItem;
  }
  return readApiItem<TItem>(result);
}

/** Reads command or scalar data from an sdkwork-v3 unwrapped payload or envelope. */
export function readUnwrappedApiData<TData>(result: unknown): TData {
  if (result && typeof result === "object" && !("code" in result) && !("traceId" in result)) {
    return result as TData;
  }
  return readApiData<TData>(result);
}
