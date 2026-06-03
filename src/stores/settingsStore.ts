import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";

export interface AppSettings {
  hotkey_mode: string;
  push_to_talk_key: string;
  toggle_key: string;
  transcription_provider: string;
  language: string;
  auto_detect_language: boolean;
  silence_timeout_ms: number;
  play_sounds: boolean;
  start_sound: boolean;
  stop_sound: boolean;
  injection_strategy: string;
  preserve_clipboard: boolean;
  appearance: string;
  compact_mode: boolean;
  translucency: boolean;
  reduced_motion: boolean;
  sound_cues: boolean;
  cloud_provider_url: string;
  cloud_provider_name: string;
}

interface SettingsStore {
  settings: AppSettings | null;
  loading: boolean;

  fetchSettings: () => Promise<void>;
  updateSetting: (key: string, value: string) => Promise<void>;
  saveAllSettings: (settings: AppSettings) => Promise<void>;
}

export const useSettingsStore = create<SettingsStore>((set) => ({
  settings: null,
  loading: false,

  fetchSettings: async () => {
    set({ loading: true });
    try {
      const settings = await invoke<AppSettings>("get_settings");
      set({ settings, loading: false });
    } catch {
      set({ loading: false });
    }
  },

  updateSetting: async (key, value) => {
    try {
      await invoke("save_setting", { key, value });
      const settings = await invoke<AppSettings>("get_settings");
      set({ settings });
    } catch {
      // ignore
    }
  },

  saveAllSettings: async (settings) => {
    try {
      await invoke("save_settings", { settings });
      set({ settings });
    } catch {
      // ignore
    }
  },
}));
