import { useEffect } from "react";
import { Zap, Globe, Mic2 } from "lucide-react";
import RecordButton from "../components/RecordButton";
import AudioVisualizer from "../components/AudioVisualizer";
import TranscriptionCard from "../components/TranscriptionCard";
import { useRecordingStore } from "../stores/recordingStore";
import { useHistoryStore } from "../stores/historyStore";
import { invoke } from "@tauri-apps/api/core";

export default function Dashboard() {
  const { mode, lastTranscription, error, clearError } = useRecordingStore();
  const { records, fetchHistory } = useHistoryStore();

  useEffect(() => {
    fetchHistory(5, 0);
  }, [fetchHistory, lastTranscription]);

  const handleCopy = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const handleResend = async (text: string) => {
    await invoke("inject_text", { text });
  };

  return (
    <div className="max-w-2xl mx-auto space-y-8">
      <div className="text-center space-y-2">
        <h1 className="text-2xl font-bold tracking-tight">Voice Dictation</h1>
        <p className="text-sm text-surface-500">
          Speak naturally. Text appears in your focused app.
        </p>
      </div>

      <div className="flex flex-col items-center py-8 space-y-4">
        <RecordButton />
        <AudioVisualizer />
      </div>

      {error && (
        <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-3 flex items-center justify-between">
          <span className="text-sm text-red-700 dark:text-red-300">
            {error}
          </span>
          <button onClick={clearError} className="text-red-500 text-xs">
            Dismiss
          </button>
        </div>
      )}

      {lastTranscription && (
        <div className="card border-accent-200 dark:border-accent-800">
          <div className="text-xs font-medium text-accent-600 dark:text-accent-400 mb-1">
            Last transcription
          </div>
          <p className="text-sm select-text">{lastTranscription}</p>
        </div>
      )}

      <div className="grid grid-cols-3 gap-3">
        <StatusTile
          icon={<Mic2 className="w-4 h-4" />}
          label="Mode"
          value={mode === "push_to_talk" ? "Push to Talk" : "Toggle"}
        />
        <StatusTile
          icon={<Zap className="w-4 h-4" />}
          label="Engine"
          value="Local Whisper"
        />
        <StatusTile
          icon={<Globe className="w-4 h-4" />}
          label="Language"
          value="English"
        />
      </div>

      {records.length > 0 && (
        <div className="space-y-3">
          <h2 className="text-sm font-semibold text-surface-700 dark:text-surface-300">
            Recent
          </h2>
          {records.slice(0, 5).map((record) => (
            <TranscriptionCard
              key={record.id}
              record={record}
              onCopy={handleCopy}
              onResend={handleResend}
            />
          ))}
        </div>
      )}
    </div>
  );
}

function StatusTile({
  icon,
  label,
  value,
}: {
  icon: React.ReactNode;
  label: string;
  value: string;
}) {
  return (
    <div className="card flex items-center gap-3">
      <div className="w-8 h-8 rounded-lg bg-surface-100 dark:bg-surface-800 flex items-center justify-center text-surface-600 dark:text-surface-400">
        {icon}
      </div>
      <div>
        <div className="text-xs text-surface-500">{label}</div>
        <div className="text-sm font-medium">{value}</div>
      </div>
    </div>
  );
}
