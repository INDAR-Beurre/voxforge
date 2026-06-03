import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";

export interface TranscriptionRecord {
  id: string;
  text: string;
  timestamp: string;
  duration_ms: number;
  word_count: number;
  mode: string;
  provider: string;
  model_name: string | null;
  language: string | null;
  target_app: string | null;
  is_favorite: boolean;
}

interface HistoryStore {
  records: TranscriptionRecord[];
  totalCount: number;
  searchQuery: string;
  loading: boolean;

  fetchHistory: (limit?: number, offset?: number) => Promise<void>;
  search: (query: string) => Promise<void>;
  toggleFavorite: (id: string) => Promise<void>;
  deleteRecord: (id: string) => Promise<void>;
  setSearchQuery: (query: string) => void;
}

export const useHistoryStore = create<HistoryStore>((set, get) => ({
  records: [],
  totalCount: 0,
  searchQuery: "",
  loading: false,

  fetchHistory: async (limit = 50, offset = 0) => {
    set({ loading: true });
    try {
      const records = await invoke<TranscriptionRecord[]>("get_history", {
        limit,
        offset,
      });
      const totalCount = await invoke<number>("get_history_count");
      set({ records, totalCount, loading: false });
    } catch {
      set({ loading: false });
    }
  },

  search: async (query) => {
    set({ loading: true, searchQuery: query });
    try {
      if (query.trim() === "") {
        await get().fetchHistory();
      } else {
        const records = await invoke<TranscriptionRecord[]>("search_history", {
          query,
          limit: 50,
        });
        set({ records, loading: false });
      }
    } catch {
      set({ loading: false });
    }
  },

  toggleFavorite: async (id) => {
    try {
      const isFav = await invoke<boolean>("toggle_favorite", { id });
      set({
        records: get().records.map((r) =>
          r.id === id ? { ...r, is_favorite: isFav } : r
        ),
      });
    } catch {
      // ignore
    }
  },

  deleteRecord: async (id) => {
    try {
      await invoke("delete_transcription", { id });
      set({ records: get().records.filter((r) => r.id !== id) });
    } catch {
      // ignore
    }
  },

  setSearchQuery: (query) => set({ searchQuery: query }),
}));
