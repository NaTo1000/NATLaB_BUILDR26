// Sandboxed Plugin System — Frontend Module
import { invoke } from "@tauri-apps/api/core";
import type { PluginInfo } from "../../types/modules";

export const PluginSystem = {
  /** List all installed plugins. */
  list: () =>
    invoke<PluginInfo[]>("plugin_list"),

  /** Install a plugin from a manifest file path. */
  install: (manifestPath: string) =>
    invoke<PluginInfo>("plugin_install", { manifestPath }),

  /** Enable a previously disabled plugin. */
  enable: (pluginId: string) =>
    invoke<void>("plugin_enable", { pluginId }),

  /** Disable (but do not uninstall) a plugin. */
  disable: (pluginId: string) =>
    invoke<void>("plugin_disable", { pluginId }),

  /** Uninstall a plugin permanently. */
  uninstall: (pluginId: string) =>
    invoke<void>("plugin_uninstall", { pluginId }),
};
