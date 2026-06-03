import { useEffect, useState } from "react";
import { Download, Trash2, CheckCircle2, HardDrive } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface WhisperModel {
  id: string;
  name: string;
  size_bytes: number;
  description: string;
  download_url: string;
  filename: string;
  recommended: boolean;
  status: ModelStatus;
}

type ModelStatus =
  | "NotDownloaded"
  | { Downloading: { progress: number } }
  | "Downloaded"
  | "Active";

export default function Models() {
  const [models, setModels] = useState<WhisperModel[]>([]);
  const [diskUsage, setDiskUsage] = useState(0);
  const [downloadProgress, setDownloadProgress] = useState<Record<string, number>>({});

  const fetchModels = async () => {
    try {
      const result = await invoke<WhisperModel[]>("get_available_models");
      setModels(result);
      const usage = await invoke<number>("get_disk_usage");
      setDiskUsage(usage);
    } catch {
      // ignore
    }
  };

  useEffect(() => {
    fetchModels();

    const unlistenProgress = listen<{ model_id: string; progress: number }>(
      "model-download-progress",
      (event) => {
        setDownloadProgress((prev) => ({
          ...prev,
          [event.payload.model_id]: event.payload.progress,
        }));
      }
    );

    const unlistenComplete = listen<{ model_id: string; success: boolean }>(
      "model-download-complete",
      (event) => {
        setDownloadProgress((prev) => {
          const next = { ...prev };
          delete next[event.payload.model_id];
          return next;
        });
        fetchModels();
      }
    );

    return () => {
      unlistenProgress.then((fn) => fn());
      unlistenComplete.then((fn) => fn());
    };
  }, []);

  const handleDownload = async (modelId: string) => {
    try {
      await invoke("download_model", { modelId });
      setDownloadProgress((prev) => ({ ...prev, [modelId]: 0 }));
    } catch {
      // ignore
    }
  };

  const handleActivate = async (modelId: string) => {
    try {
      await invoke("set_active_model", { modelId });
      fetchModels();
    } catch {
      // ignore
    }
  };

  const handleDelete = async (modelId: string) => {
    try {
      await invoke("delete_model", { modelId });
      fetchModels();
    } catch {
      // ignore
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-bold">Whisper Models</h1>
          <p className="text-sm text-surface-500 mt-1">
            Manage local transcription models. Larger models are more accurate
            but slower.
          </p>
        </div>
        <div className="flex items-center gap-2 text-sm text-surface-500">
          <HardDrive className="w-4 h-4" />
          {formatBytes(diskUsage)} used
        </div>
      </div>

      <div className="space-y-3">
        {models.map((model) => {
          const progress = downloadProgress[model.id];
          const isDownloading = progress !== undefined;

          return (
            <div key={model.id} className="card">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div className="flex flex-col">
                    <div className="flex items-center gap-2">
                      <span className="text-sm font-semibold">
                        {model.name}
                      </span>
                      {model.recommended && (
                        <span className="px-1.5 py-0.5 bg-accent-100 dark:bg-accent-900/30 text-accent-700 dark:text-accent-300 rounded text-[10px] font-medium">
                          Recommended
                        </span>
                      )}
                      {model.status === "Active" && (
                        <span className="px-1.5 py-0.5 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300 rounded text-[10px] font-medium">
                          Active
                        </span>
                      )}
                    </div>
                    <span className="text-xs text-surface-500 mt-0.5">
                      {model.description}
                    </span>
                  </div>
                </div>

                <div className="flex items-center gap-3">
                  <span className="text-xs text-surface-400">
                    {formatBytes(model.size_bytes)}
                  </span>

                  {model.status === "NotDownloaded" && !isDownloading && (
                    <button
                      onClick={() => handleDownload(model.id)}
                      className="btn-primary flex items-center gap-1.5 text-xs py-1.5 px-3"
                    >
                      <Download className="w-3.5 h-3.5" />
                      Download
                    </button>
                  )}

                  {isDownloading && (
                    <div className="flex items-center gap-2">
                      <div className="w-24 h-1.5 bg-surface-200 dark:bg-surface-700 rounded-full overflow-hidden">
                        <div
                          className="h-full bg-accent-500 rounded-full transition-all"
                          style={{ width: `${(progress ?? 0) * 100}%` }}
                        />
                      </div>
                      <span className="text-xs text-surface-500">
                        {Math.round((progress ?? 0) * 100)}%
                      </span>
                    </div>
                  )}

                  {model.status === "Downloaded" && (
                    <div className="flex items-center gap-2">
                      <button
                        onClick={() => handleActivate(model.id)}
                        className="btn-secondary text-xs py-1.5 px-3"
                      >
                        Activate
                      </button>
                      <button
                        onClick={() => handleDelete(model.id)}
                        className="btn-ghost p-1.5 hover:text-red-500"
                      >
                        <Trash2 className="w-3.5 h-3.5" />
                      </button>
                    </div>
                  )}

                  {model.status === "Active" && (
                    <CheckCircle2 className="w-5 h-5 text-green-500" />
                  )}
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const units = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${units[i]}`;
}
