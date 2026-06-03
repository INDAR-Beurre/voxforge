import { Copy, Star, Trash2, RotateCcw } from "lucide-react";
import clsx from "clsx";
import type { TranscriptionRecord } from "../stores/historyStore";

interface Props {
  record: TranscriptionRecord;
  onCopy?: (text: string) => void;
  onFavorite?: (id: string) => void;
  onDelete?: (id: string) => void;
  onResend?: (text: string) => void;
}

export default function TranscriptionCard({
  record,
  onCopy,
  onFavorite,
  onDelete,
  onResend,
}: Props) {
  const timeAgo = formatTimeAgo(record.timestamp);

  return (
    <div className="card group hover:border-accent-200 dark:hover:border-accent-800 transition-colors">
      <div className="flex items-start justify-between gap-3">
        <p className="text-sm leading-relaxed flex-1 select-text">
          {record.text}
        </p>
        <div className="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
          {onCopy && (
            <button
              onClick={() => onCopy(record.text)}
              className="btn-ghost p-1.5"
              title="Copy"
            >
              <Copy className="w-3.5 h-3.5" />
            </button>
          )}
          {onResend && (
            <button
              onClick={() => onResend(record.text)}
              className="btn-ghost p-1.5"
              title="Re-inject"
            >
              <RotateCcw className="w-3.5 h-3.5" />
            </button>
          )}
          {onFavorite && (
            <button
              onClick={() => onFavorite(record.id)}
              className="btn-ghost p-1.5"
              title="Favorite"
            >
              <Star
                className={clsx(
                  "w-3.5 h-3.5",
                  record.is_favorite && "fill-yellow-400 text-yellow-400"
                )}
              />
            </button>
          )}
          {onDelete && (
            <button
              onClick={() => onDelete(record.id)}
              className="btn-ghost p-1.5 hover:text-red-500"
              title="Delete"
            >
              <Trash2 className="w-3.5 h-3.5" />
            </button>
          )}
        </div>
      </div>

      <div className="flex items-center gap-3 mt-2 text-xs text-surface-500">
        <span>{timeAgo}</span>
        <span>{record.word_count} words</span>
        <span>{formatDuration(record.duration_ms)}</span>
        {record.target_app && (
          <span className="px-1.5 py-0.5 bg-surface-100 dark:bg-surface-800 rounded text-xs">
            {record.target_app}
          </span>
        )}
      </div>
    </div>
  );
}

function formatTimeAgo(timestamp: string): string {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();
  const minutes = Math.floor(diff / 60000);

  if (minutes < 1) return "Just now";
  if (minutes < 60) return `${minutes}m ago`;
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h ago`;
  const days = Math.floor(hours / 24);
  if (days < 7) return `${days}d ago`;
  return date.toLocaleDateString();
}

function formatDuration(ms: number): string {
  if (ms < 1000) return `${ms}ms`;
  const seconds = Math.floor(ms / 1000);
  if (seconds < 60) return `${seconds}s`;
  const minutes = Math.floor(seconds / 60);
  return `${minutes}m ${seconds % 60}s`;
}
