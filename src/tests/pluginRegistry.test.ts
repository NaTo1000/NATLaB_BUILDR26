// Unit tests: PluginRegistry — registration, enable/disable, permissions.

import { describe, it, expect, vi, beforeEach } from "vitest";
import { PluginRegistry } from "../core/PluginRegistry";
import type { PluginManifest } from "../types/modules";

function makeManifest(id: string, permissions: string[] = []): PluginManifest {
  return {
    id,
    name: `Plugin ${id}`,
    version: "1.0.0",
    author: "Test",
    description: "Test plugin",
    entry_point: "index.js",
    permissions,
    min_natlab_version: "0.1.0",
    checksum_sha256: "abc123",
  };
}

describe("PluginRegistry", () => {
  let registry: PluginRegistry;

  beforeEach(() => {
    registry = new PluginRegistry();
  });

  it("registers a plugin as 'available'", () => {
    registry.register(makeManifest("plugin-a"));
    const plugins = registry.list();
    const p = plugins.find((x) => x.manifest.id === "plugin-a");
    expect(p).toBeDefined();
    expect(p!.status).toBe("available");
  });

  it("throws when registering a duplicate plugin", () => {
    registry.register(makeManifest("plugin-b"));
    expect(() => registry.register(makeManifest("plugin-b"))).toThrow(
      "Plugin 'plugin-b' is already registered"
    );
  });

  it("enables a plugin and calls the onLoad and onEnable hooks", async () => {
    const onLoad = vi.fn().mockResolvedValue(undefined);
    const onEnable = vi.fn().mockResolvedValue(undefined);
    registry.register(makeManifest("plugin-c"), { onLoad, onEnable });
    await registry.enable("plugin-c");

    const plugins = registry.list();
    expect(plugins.find((p) => p.manifest.id === "plugin-c")!.status).toBe("loaded");
    expect(onLoad).toHaveBeenCalledOnce();
    expect(onEnable).toHaveBeenCalledOnce();
  });

  it("disables a plugin and calls the onDisable hook", async () => {
    const onDisable = vi.fn().mockResolvedValue(undefined);
    registry.register(makeManifest("plugin-d"), { onDisable });
    await registry.enable("plugin-d");
    await registry.disable("plugin-d");

    const plugins = registry.list();
    expect(plugins.find((p) => p.manifest.id === "plugin-d")!.status).toBe("disabled");
    expect(onDisable).toHaveBeenCalledOnce();
  });

  it("unloads a plugin and removes it from the registry", async () => {
    const onUnload = vi.fn().mockResolvedValue(undefined);
    registry.register(makeManifest("plugin-e"), { onUnload });
    await registry.unload("plugin-e");

    const plugins = registry.list();
    expect(plugins.find((p) => p.manifest.id === "plugin-e")).toBeUndefined();
    expect(onUnload).toHaveBeenCalledOnce();
  });

  it("correctly checks permissions", () => {
    registry.register(makeManifest("plugin-f", ["fs:read", "network:fetch"]));
    expect(registry.hasPermission("plugin-f", "fs:read")).toBe(true);
    expect(registry.hasPermission("plugin-f", "disk:write")).toBe(false);
  });

  it("returns false for permissions on unknown plugin", () => {
    expect(registry.hasPermission("non-existent", "fs:read")).toBe(false);
  });

  it("throws when operating on an unregistered plugin", async () => {
    await expect(registry.enable("ghost")).rejects.toThrow(
      "Plugin 'ghost' is not registered"
    );
  });
});
