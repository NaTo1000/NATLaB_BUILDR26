// Core Engine — Plugin Registry
// Manages third-party plugin lifecycle: registration, permission checks, sandboxing.

import type { PluginManifest, PluginInfo, PluginStatus } from "../types/modules";

export interface PluginHooks {
  onLoad?: () => Promise<void>;
  onUnload?: () => Promise<void>;
  onEnable?: () => Promise<void>;
  onDisable?: () => Promise<void>;
}

export interface RegisteredPlugin {
  manifest: PluginManifest;
  hooks: PluginHooks;
  status: PluginStatus;
  sandboxWindow: Window | null;
  loadedAt: Date | null;
}

export class PluginRegistry {
  private readonly plugins = new Map<string, RegisteredPlugin>();

  /** Register a plugin with its manifest and lifecycle hooks. */
  register(manifest: PluginManifest, hooks: PluginHooks = {}): void {
    if (this.plugins.has(manifest.id)) {
      throw new Error(`Plugin '${manifest.id}' is already registered`);
    }
    this.plugins.set(manifest.id, {
      manifest,
      hooks,
      status: "available",
      sandboxWindow: null,
      loadedAt: null,
    });
  }

  /** Load and enable a plugin by ID. */
  async enable(id: string): Promise<void> {
    const plugin = this.getOrThrow(id);
    if (plugin.status === "loaded") return;

    plugin.status = "loaded";
    plugin.loadedAt = new Date();
    await plugin.hooks.onEnable?.();
  }

  /** Disable a plugin without unregistering it. */
  async disable(id: string): Promise<void> {
    const plugin = this.getOrThrow(id);
    if (plugin.status === "disabled") return;

    await plugin.hooks.onDisable?.();
    plugin.status = "disabled";
  }

  /** Unregister and clean up a plugin. */
  async unload(id: string): Promise<void> {
    const plugin = this.getOrThrow(id);
    await plugin.hooks.onUnload?.();
    plugin.sandboxWindow?.close();
    this.plugins.delete(id);
  }

  /** Check whether a plugin has a specific permission. */
  hasPermission(id: string, permission: string): boolean {
    const plugin = this.plugins.get(id);
    return plugin?.manifest.permissions.includes(permission) ?? false;
  }

  /** Return all registered plugins as PluginInfo objects. */
  list(): PluginInfo[] {
    return Array.from(this.plugins.values()).map((p) => ({
      manifest: p.manifest,
      status: p.status,
      loaded_at: p.loadedAt?.toISOString() ?? null,
      sandbox_id: null,
    }));
  }

  private getOrThrow(id: string): RegisteredPlugin {
    const plugin = this.plugins.get(id);
    if (!plugin) throw new Error(`Plugin '${id}' is not registered`);
    return plugin;
  }
}

export const pluginRegistry = new PluginRegistry();
