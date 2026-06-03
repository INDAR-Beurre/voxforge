import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";

export type RecordingState = "idle" | "recording" | "processing";
export type HotkeyMode = "push_to_talk" | "toggle";

interface RecordingStore {
  state: RecordingState;
  mode: HotkeyMode;
  level: number;
  lastTranscription: string | null;
  error: string | null;

  startRecording: () => Promise<void>;
  stopRecording: () => Promise<void>;
  transcribeAndInject: () => Promise<void>;
  setMode: (mode: HotkeyMode) => void;
  updateLevel: () => Promise<void>;
  clearError: () => void;
}

export const useRecordingStore = create<RecordingStore>((set, get) => ({
  state: "idle",
  mode: "push_to_talk",
  level: 0,
  lastTranscription: null,
  error: null,

  startRecording: async () => {
    try {
      await invoke("start_recording");
      set({ state: "recording", error: null });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  stopRecording: async () => {
    try {
      await invoke("stop_recording");
      set({ state: "idle" });
    } catch (e) {
      set({ error: String(e) });
    }
  },

  transcribeAndInject: async () => {
    try {
      set({ state: "processing" });
      const result = await invoke<{ text: string }>("transcribe_and_inject");
      set({ state: "idle", lastTranscription: result.text });
    } catch (e) {
      set({ state: "idle", error: String(e) });
    }
  },

  setMode: (mode) => set({ mode }),

  updateLevel: async () => {
    if (get().state !== "recording") return;
    try {
      const level = await invoke<number>("get_audio_level");
      set({ level });
    } catch {
      // ignore
    }
  },

  clearError: () => set({ error: null }),
}));
