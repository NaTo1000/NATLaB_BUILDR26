// Core Engine — Lazy Loader
// Defers module initialisation until first use, tracking load time for perf analysis.

import { invoke } from "@tauri-apps/api/core";
import type { ModuleDescriptor } from "../types/modules";

type ModuleFactory<T> = () => Promise<T>;

interface LoadEntry<T> {
  factory: ModuleFactory<T>;
  instance: T | null;
  loadDurationMs: number | null;
  loading: boolean;
  error: string | null;
}

/** Frontend lazy loader — mirrors the backend ModuleRegistry. */
export class LazyLoader {
  private readonly entries = new Map<string, LoadEntry<unknown>>();

  /** Register a module factory without loading it. */
  register<T>(id: string, factory: ModuleFactory<T>): void {
    if (this.entries.has(id)) return;
    this.entries.set(id, {
      factory: factory as ModuleFactory<unknown>,
      instance: null,
      loadDurationMs: null,
      loading: false,
      error: null,
    });
  }

  /** Load a module on demand and cache the instance. */
  async load<T>(id: string): Promise<T> {
    const entry = this.entries.get(id);
    if (!entry) throw new Error(`Module '${id}' is not registered`);

    if (entry.instance !== null) return entry.instance as T;
    if (entry.loading) {
      // Wait for the in-flight load to complete
      await new Promise<void>((resolve) => {
        const check = setInterval(() => {
          if (!entry.loading) {
            clearInterval(check);
            resolve();
          }
        }, 16);
      });
      if (entry.error) {
        throw new Error(entry.error);
      }
      if (entry.instance === null) {
        throw new Error(`Module '${id}' failed to load`);
      }
      return entry.instance as T;
    }

    entry.loading = true;
    entry.error = null;
    const startTime = performance.now();

    try {
      // Notify backend to mark the module as loaded
      await invoke<ModuleDescriptor>("core_load_module", { moduleId: id });
      entry.instance = await entry.factory();
      entry.loadDurationMs = performance.now() - startTime;
      return entry.instance as T;
    } catch (err) {
      entry.error = String(err);
      throw err;
    } finally {
      entry.loading = false;
    }
  }

  /** Return whether a module has been loaded. */
  isLoaded(id: string): boolean {
    return this.entries.get(id)?.instance !== null;
  }

  /** Return load duration in milliseconds, or null if not yet loaded. */
  loadTime(id: string): number | null {
    return this.entries.get(id)?.loadDurationMs ?? null;
  }

  /** Return all registered module IDs. */
  registeredIds(): string[] {
    return Array.from(this.entries.keys());
  }
}

export const lazyLoader = new LazyLoader();
