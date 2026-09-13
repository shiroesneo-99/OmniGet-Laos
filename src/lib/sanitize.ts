import DOMPurify from "dompurify";

type Purifier = ReturnType<typeof DOMPurify>;

let purifier: Purifier | null = null;

// Private instance so the link hook below never leaks into other DOMPurify
// users (mermaid ships its own sanitization pass).
function getPurifier(): Purifier {
  if (purifier) return purifier;
  const p = DOMPurify(window);
  p.addHook("afterSanitizeAttributes", (node) => {
    if (node.tagName !== "A") return;
    const href = node.getAttribute("href")?.trim() ?? "";
    if (/^https?:/i.test(href)) {
      node.setAttribute("target", "_blank");
      node.setAttribute("rel", "noopener noreferrer");
    }
  });
  purifier = p;
  return p;
}

/// Sanitizes untrusted HTML (course/lesson descriptions, rendered markdown)
/// before it reaches `{@html}`. DOMPurify already strips event handlers and
/// `javascript:` URLs; embedding/active elements are forbidden explicitly and
/// external links open outside the app window.
export function sanitizeHtml(html: string): string {
  if (!html) return "";
  return getPurifier().sanitize(html, {
    USE_PROFILES: { html: true },
    FORBID_TAGS: ["script", "style", "iframe", "object", "embed", "form", "base", "meta", "link"],
  });
}

/// Sanitizes an SVG document (e.g. a PlantUML render) before it is written to
/// `innerHTML`: keeps shapes and filters, drops scripts, handlers and
/// `<foreignObject>` HTML.
export function sanitizeSvg(svg: string): string {
  if (!svg) return "";
  return getPurifier().sanitize(svg, {
    USE_PROFILES: { svg: true, svgFilters: true },
    FORBID_TAGS: ["script", "foreignObject"],
  });
}

export function escapeHtml(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}
