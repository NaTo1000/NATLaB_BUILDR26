// Cloud Sync / Profiles — Frontend Module
import { invoke } from "@tauri-apps/api/core";
import type { UserProfile, SyncStatus, SyncResult } from "../../types/modules";

export const CloudSync = {
  /** Get the current user profile. */
  getProfile: () =>
    invoke<UserProfile | null>("cloud_get_profile"),

  /** Update user preferences. */
  updatePreferences: (preferences: Record<string, unknown>) =>
    invoke<void>("cloud_update_preferences", { preferences }),

  /** Trigger an immediate cloud sync. */
  syncNow: () =>
    invoke<SyncResult>("cloud_sync_now"),

  /** Get the current sync status. */
  status: () =>
    invoke<SyncStatus>("cloud_sync_status"),
};
