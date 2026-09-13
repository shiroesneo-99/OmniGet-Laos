/**
 * Opening things outside the webview: reveal a file in its folder, open a
 * file/folder with the default app, or open an external link in the browser.
 *
 * Goes through what the app already ships — the `reveal_file` /
 * `open_path_default` commands and the shell plugin's `open` (whose default
 * scope only allows http(s)/mailto/tel) — instead of a separate opener plugin.
 * These throw on failure; `$lib/tools/rt` wraps them with an error toast.
 */
import { invoke } from "@tauri-apps/api/core";

const ALLOWED_URL_SCHEMES = new Set(["http:", "https:", "mailto:"]);

/** Returns the normalized URL when it is http(s)/mailto, otherwise `null`. */
export function safeExternalUrl(url: string): string | null {
  let parsed: URL;
  try {
    parsed = new URL(url.trim());
  } catch {
    return null;
  }
  return ALLOWED_URL_SCHEMES.has(parsed.protocol) ? parsed.href : null;
}

export async function revealPath(path: string): Promise<void> {
  if (!path) throw new Error("No path to reveal");
  await invoke("reveal_file", { path });
}

export async function openLocalPath(path: string): Promise<void> {
  if (!path) throw new Error("No path to open");
  await invoke("open_path_default", { path });
}

export async function openExternalUrl(url: string): Promise<void> {
  const safe = safeExternalUrl(url);
  if (!safe) throw new Error(`Refusing to open URL: ${url}`);
  const { open } = await import("@tauri-apps/plugin-shell");
  await open(safe);
}
