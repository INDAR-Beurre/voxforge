import { Copy, Star, Trash2, RotateCcw } from "lucide-react";
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
    <div className="card group" style={{ padding: "16px 20px", transition: "border-color 0.15s ease" }}>
      <div className="flex items-start justify-between gap-3">
        <p
          className="flex-1 select-text"
          style={{
            fontSize: "14px",
            lineHeight: "1.6",
            color: "var(--color-text)",
          }}
        >
          {record.text}
        </p>
        <div
          className="flex items-center gap-0.5"
          style={{ opacity: 0, transition: "opacity 0.12s ease" }}
        >
          {onCopy && (
            <button onClick={() => onCopy(record.text)} className="btn-ghost" title="Copy">
              <Copy style={{ width: "14px", height: "14px" }} />
            </button>
          )}
          {onResend && (
            <button onClick={() => onResend(record.text)} className="btn-ghost" title="Re-inject">
              <RotateCcw style={{ width: "14px", height: "14px" }} />
            </button>
          )}
          {onFavorite && (
            <button onClick={() => onFavorite(record.id)} className="btn-ghost" title="Favorite">
              <Star
                style={{
                  width: "14px",
                  height: "14px",
                  fill: record.is_favorite ? "#ffb005" : "none",
                  color: record.is_favorite ? "#ffb005" : "currentColor",
                }}
              />
            </button>
          )}
          {onDelete && (
            <button onClick={() => onDelete(record.id)} className="btn-ghost" title="Delete">
              <Trash2 style={{ width: "14px", height: "14px" }} />
            </button>
          )}
        </div>
      </div>

      <div
        className="flex items-center gap-3 mt-2.5"
        style={{ fontSize: "12px", color: "var(--color-text-tertiary)" }}
      >
        <span>{timeAgo}</span>
        <span>{record.word_count} words</span>
        <span>{formatDuration(record.duration_ms)}</span>
        {record.target_app && (
          <span
            style={{
              padding: "2px 8px",
              borderRadius: "6px",
              background: "var(--color-fill-secondary)",
              fontSize: "11px",
            }}
          >
            {record.target_app}
          </span>
        )}
      </div>

      <style>{`
        .card:hover .flex > div[style*="opacity: 0"] {
          opacity: 1 !important;
        }
        .group:hover div[style*="opacity: 0"] {
          opacity: 1 !important;
        }
      `}</style>
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
