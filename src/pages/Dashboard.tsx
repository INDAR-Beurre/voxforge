import { useEffect, useState } from "react";
import { Zap, Globe, Mic2, AlertTriangle, ExternalLink, Download } from "lucide-react";
import { useNavigate } from "react-router-dom";
import RecordButton from "../components/RecordButton";
import AudioVisualizer from "../components/AudioVisualizer";
import TranscriptionCard from "../components/TranscriptionCard";
import { useRecordingStore } from "../stores/recordingStore";
import { useHistoryStore } from "../stores/historyStore";
import { invoke } from "@tauri-apps/api/core";

interface WhisperModel {
  id: string;
  name: string;
  status: string | { Downloading: { progress: number } };
}

export default function Dashboard() {
  const navigate = useNavigate();
  const { mode, lastTranscription, error, clearError } = useRecordingStore();
  const { records, fetchHistory } = useHistoryStore();
  const [hasAccessibility, setHasAccessibility] = useState<boolean | null>(null);
  const [hasMicrophone, setHasMicrophone] = useState<boolean | null>(null);
  const [hasModel, setHasModel] = useState<boolean | null>(null);

  useEffect(() => {
    fetchHistory(5, 0);
  }, [fetchHistory, lastTranscription]);

  useEffect(() => {
    invoke<boolean>("check_accessibility_permission")
      .then(setHasAccessibility)
      .catch(() => setHasAccessibility(false));

    invoke<boolean>("request_microphone_permission")
      .then(setHasMicrophone)
      .catch(() => setHasMicrophone(false));

    invoke<WhisperModel[]>("get_available_models")
      .then((models) => {
        const active = models.some(
          (m) => m.status === "Active" || m.status === "Downloaded"
        );
        setHasModel(active);
      })
      .catch(() => setHasModel(false));
  }, []);

  const handleCopy = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const handleResend = async (text: string) => {
    await invoke("inject_text", { text });
  };

  const openAccessibility = async () => {
    await invoke("request_accessibility_permission");
    setTimeout(() => {
      invoke<boolean>("check_accessibility_permission").then(setHasAccessibility);
    }, 2000);
  };

  const openMicrophone = () => {
    invoke("open_microphone_settings");
  };

  return (
    <div className="max-w-2xl mx-auto space-y-8">
      {hasModel === false && (
        <div className="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-xl p-4">
          <div className="flex items-start gap-3">
            <Download className="w-5 h-5 text-blue-600 dark:text-blue-400 shrink-0 mt-0.5" />
            <div className="flex-1">
              <h3 className="text-sm font-semibold text-blue-800 dark:text-blue-200">
                Download a Transcription Model
              </h3>
              <p className="text-xs text-blue-700 dark:text-blue-300 mt-1">
                You need to download a Whisper model before VoxForge can transcribe speech. Base (142 MB) is recommended.
              </p>
              <button
                onClick={() => navigate("/models")}
                className="btn-primary text-xs py-1.5 px-3 mt-3"
              >
                Go to Models
              </button>
            </div>
          </div>
        </div>
      )}

      {hasMicrophone === false && (
        <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-xl p-4">
          <div className="flex items-start gap-3">
            <Mic2 className="w-5 h-5 text-red-600 dark:text-red-400 shrink-0 mt-0.5" />
            <div className="flex-1">
              <h3 className="text-sm font-semibold text-red-800 dark:text-red-200">
                Microphone Permission Required
              </h3>
              <p className="text-xs text-red-700 dark:text-red-300 mt-1">
                VoxForge can't access your microphone. Grant permission in System Settings.
              </p>
              <button
                onClick={openMicrophone}
                className="btn-primary text-xs py-1.5 px-3 mt-3 flex items-center gap-1.5 bg-red-600 hover:bg-red-700"
              >
                <ExternalLink className="w-3.5 h-3.5" />
                Open Microphone Settings
              </button>
            </div>
          </div>
        </div>
      )}

      {hasAccessibility === false && (
        <div className="bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-xl p-4">
          <div className="flex items-start gap-3">
            <AlertTriangle className="w-5 h-5 text-amber-600 dark:text-amber-400 shrink-0 mt-0.5" />
            <div className="flex-1">
              <h3 className="text-sm font-semibold text-amber-800 dark:text-amber-200">
                Accessibility Permission Required
              </h3>
              <p className="text-xs text-amber-700 dark:text-amber-300 mt-1">
                VoxForge needs Accessibility access for global shortcuts and text injection.
              </p>
              <div className="flex gap-2 mt-3">
                <button onClick={openAccessibility} className="btn-primary text-xs py-1.5 px-3 flex items-center gap-1.5">
                  <ExternalLink className="w-3.5 h-3.5" />
                  Grant Accessibility Access
                </button>
                <button onClick={openMicrophone} className="btn-secondary text-xs py-1.5 px-3 flex items-center gap-1.5">
                  <ExternalLink className="w-3.5 h-3.5" />
                  Microphone Settings
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

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
          value={hasModel ? "Whisper Ready" : "No Model"}
        />
        <StatusTile
          icon={<Globe className="w-4 h-4" />}
          label="Language"
          value="Auto (EN/FR/DE)"
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
