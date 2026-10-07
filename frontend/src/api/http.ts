export class HttpError extends Error {
  readonly status: number;
  readonly code: string | null;

  constructor(status: number, message: string, code: string | null = null) {
    super(message);
    this.name = "HttpError";
    this.status = status;
    this.code = code;
  }
}

function errorMessage(body: unknown): string | null {
  if (typeof body !== "object" || body === null || !("error" in body)) {
    return null;
  }

  const error = body.error;

  if (typeof error !== "object" || error === null || !("message" in error)) {
    return null;
  }

  return typeof error.message === "string" ? error.message : null;
}

export async function getJson<T>(
  path: string,
  signal?: AbortSignal,
): Promise<T> {
  return request<T>(path, "GET", undefined, signal);
}

let csrfToken: string | null = null;
let csrfRequest: Promise<string> | null = null;

export function clearCsrf() {
  csrfToken = null;
}

async function csrf(): Promise<string> {
  if (csrfToken !== null) {
    return csrfToken;
  }
  if (csrfRequest === null) {
    csrfRequest = request<{ csrfToken: string }>("/api/auth/csrf", "GET")
      .then((result) => {
        csrfToken = result.csrfToken;
        return result.csrfToken;
      })
      .finally(() => {
        csrfRequest = null;
      });
  }
  return csrfRequest;
}

export async function mutateJson<T = void>(
  path: string,
  method: "POST" | "PUT" | "PATCH" | "DELETE",
  body?: unknown,
  signal?: AbortSignal,
): Promise<T> {
  const token = await csrf();
  try {
    return await request<T>(path, method, body, signal, token);
  } catch (reason) {
    if (reason instanceof HttpError && reason.code === "csrf") {
      clearCsrf();
    }
    throw reason;
  }
}

async function request<T>(
  path: string,
  method: string,
  body?: unknown,
  signal?: AbortSignal,
  csrf?: string,
): Promise<T> {
  let response: Response;
  const headers: Record<string, string> = { Accept: "application/json" };
  if (body !== undefined) {
    headers["Content-Type"] = "application/json";
  }
  if (csrf !== undefined) {
    headers["X-CSRF-Token"] = csrf;
  }

  try {
    response = await fetch(path, {
      method,
      headers,
      credentials: "same-origin",
      body: body === undefined ? undefined : JSON.stringify(body),
      signal,
    });
  } catch (reason) {
    signal?.throwIfAborted();

    if (reason instanceof TypeError) {
      throw new Error(
        "Не удалось связаться с сервером. Проверьте, что backend запущен.",
      );
    }

    throw reason;
  }

  if (!response.ok) {
    const body: unknown = await response.json().catch(() => null);
    const message =
      errorMessage(body) ??
      `Ошибка сервера (HTTP ${response.status}). Повторите загрузку.`;
    let code: string | null = null;
    if (typeof body === "object" && body !== null && "error" in body) {
      const error = body.error;
      if (
        typeof error === "object" &&
        error !== null &&
        "code" in error &&
        typeof error.code === "string"
      ) {
        code = error.code;
      }
    }
    throw new HttpError(response.status, message, code);
  }
  if (response.status === 204) {
    return undefined as T;
  }
  return response.json() as Promise<T>;
}
