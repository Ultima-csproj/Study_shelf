export class HttpError extends Error {
  readonly status: number;

  constructor(status: number, message: string) {
    super(message);
    this.name = "HttpError";
    this.status = status;
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
  let response: Response;

  try {
    response = await fetch(path, {
      headers: { Accept: "application/json" },
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
    throw new HttpError(response.status, message);
  }

  return response.json() as Promise<T>;
}
