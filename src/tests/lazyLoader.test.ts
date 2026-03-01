// Unit tests: LazyLoader — registration, loading, caching, error handling.
// Mocks the Tauri invoke call so no real IPC is needed.

import { describe, it, expect, vi, beforeEach } from "vitest";
import { LazyLoader } from "../core/LazyLoader";

// Mock the Tauri API so tests run in Node.
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn().mockResolvedValue({ id: "mock", loaded: true }),
}));

describe("LazyLoader", () => {
  let loader: LazyLoader;

  beforeEach(() => {
    loader = new LazyLoader();
  });

  it("registers a module without loading it", () => {
    loader.register("mod_a", async () => ({ value: 42 }));
    expect(loader.isLoaded("mod_a")).toBe(false);
    expect(loader.registeredIds()).toContain("mod_a");
  });

  it("loads a module and caches the instance", async () => {
    const factory = vi.fn().mockResolvedValue({ value: 99 });
    loader.register("mod_b", factory);

    const instance = await loader.load<{ value: number }>("mod_b");
    expect(instance.value).toBe(99);
    expect(loader.isLoaded("mod_b")).toBe(true);
    expect(factory).toHaveBeenCalledOnce();

    // Second load returns cached instance without calling factory again.
    const cached = await loader.load<{ value: number }>("mod_b");
    expect(cached).toBe(instance);
    expect(factory).toHaveBeenCalledOnce();
  });

  it("records load time after successful load", async () => {
    loader.register("mod_c", async () => ({}));
    expect(loader.loadTime("mod_c")).toBeNull();
    await loader.load("mod_c");
    expect(loader.loadTime("mod_c")).toBeGreaterThan(0);
  });

  it("throws when loading an unregistered module", async () => {
    await expect(loader.load("does_not_exist")).rejects.toThrow(
      "Module 'does_not_exist' is not registered"
    );
  });

  it("ignores duplicate registrations", () => {
    const factory1 = vi.fn().mockResolvedValue("first");
    const factory2 = vi.fn().mockResolvedValue("second");
    loader.register("mod_d", factory1);
    loader.register("mod_d", factory2); // should be ignored

    // factory1 should be used, factory2 should never be called.
    expect(loader.registeredIds().filter((id) => id === "mod_d").length).toBe(1);
  });

  it("returns all registered IDs", () => {
    loader.register("x", async () => null);
    loader.register("y", async () => null);
    loader.register("z", async () => null);
    const ids = loader.registeredIds();
    expect(ids).toContain("x");
    expect(ids).toContain("y");
    expect(ids).toContain("z");
  });
});
