import { describe, it, expect } from "vitest";
import { openExternalUrl, safeExternalUrl } from "./open";

describe("safeExternalUrl", () => {
  it("accepts http, https and mailto", () => {
    expect(safeExternalUrl("https://www.spotify.com/download/")).toBe("https://www.spotify.com/download/");
    expect(safeExternalUrl(" http://example.com ")).toBe("http://example.com/");
    expect(safeExternalUrl("mailto:someone@example.com")).toBe("mailto:someone@example.com");
  });

  it("rejects other schemes and garbage", () => {
    expect(safeExternalUrl("javascript:alert(1)")).toBeNull();
    expect(safeExternalUrl("file:///C:/Windows/System32/calc.exe")).toBeNull();
    expect(safeExternalUrl("C:\\Users\\me\\file.txt")).toBeNull();
    expect(safeExternalUrl("tel:123")).toBeNull();
    expect(safeExternalUrl("")).toBeNull();
    expect(safeExternalUrl("not a url")).toBeNull();
  });
});

describe("openExternalUrl", () => {
  it("throws before reaching the shell for disallowed URLs", async () => {
    await expect(openExternalUrl("javascript:alert(1)")).rejects.toThrow(/Refusing/);
  });
});
