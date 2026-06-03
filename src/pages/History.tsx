import { useEffect, useState } from "react";
import { Search, Download } from "lucide-react";
import TranscriptionCard from "../components/TranscriptionCard";
import { useHistoryStore } from "../stores/historyStore";
import { invoke } from "@tauri-apps/api/core";

export default function History() {
  const { records, searchQuery, loading, fetchHistory, search, toggleFavorite, deleteRecord, setSearchQuery } =
    useHistoryStore();
  const [filter, setFilter] = useState<"all" | "favorites">("all");

  useEffect(() => {
    fetchHistory(100, 0);
  }, [fetchHistory]);

  const handleSearch = (value: string) => {
    setSearchQuery(value);
    search(value);
  };

  const handleCopy = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const handleResend = async (text: string) => {
    await invoke("inject_text", { text });
  };

  const filteredRecords =
    filter === "favorites" ? records.filter((r) => r.is_favorite) : records;

  const handleExport = () => {
    const data = JSON.stringify(records, null, 2);
    const blob = new Blob([data], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `voxforge-history-${new Date().toISOString().slice(0, 10)}.json`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-bold">Transcription History</h1>
        <button onClick={handleExport} className="btn-secondary flex items-center gap-2">
          <Download className="w-4 h-4" />
          Export
        </button>
      </div>

      <div className="flex items-center gap-3">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-surface-400" />
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => handleSearch(e.target.value)}
            placeholder="Search transcriptions..."
            className="input pl-9"
          />
        </div>
        <div className="flex items-center gap-1 bg-surface-100 dark:bg-surface-800 rounded-lg p-0.5">
          <button
            onClick={() => setFilter("all")}
            className={`px-3 py-1.5 rounded-md text-xs font-medium transition-colors ${
              filter === "all"
                ? "bg-white dark:bg-surface-700 shadow-sm"
                : "text-surface-600 dark:text-surface-400"
            }`}
          >
            All
          </button>
          <button
            onClick={() => setFilter("favorites")}
            className={`px-3 py-1.5 rounded-md text-xs font-medium transition-colors ${
              filter === "favorites"
                ? "bg-white dark:bg-surface-700 shadow-sm"
                : "text-surface-600 dark:text-surface-400"
            }`}
          >
            Favorites
          </button>
        </div>
      </div>

      {loading ? (
        <div className="text-center py-12 text-surface-500">Loading...</div>
      ) : filteredRecords.length === 0 ? (
        <div className="text-center py-12">
          <p className="text-surface-500">No transcriptions yet.</p>
          <p className="text-xs text-surface-400 mt-1">
            Your voice dictation history will appear here.
          </p>
        </div>
      ) : (
        <div className="space-y-3">
          {filteredRecords.map((record) => (
            <TranscriptionCard
              key={record.id}
              record={record}
              onCopy={handleCopy}
              onResend={handleResend}
              onFavorite={toggleFavorite}
              onDelete={deleteRecord}
            />
          ))}
        </div>
      )}
    </div>
  );
}
