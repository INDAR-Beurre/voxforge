import { useEffect, useState } from "react";
import { AlertTriangle, ExternalLink, Download } from "lucide-react";
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
  const { lastTranscription, error, clearError } = useRecordingStore();
  const { records, fetchHistory } = useHistoryStore();
  const [hasAccessibility, setHasAccessibility] = useState<boolean | null>(null);
  const [, setHasMicrophone] = useState<boolean | null>(null);
  const [hasModel, setHasModel] = useState<boolean | null>(null);

  useEffect(() => {
    fetchHistory(5, 0);
  }, [fetchHistory, lastTranscription]);

  useEffect(() => {
    invoke<boolean>("check_accessibility_permission")
      .then(setHasAccessibility)
      .catch(() => setHasAccessibility(false));

    invoke<string>("check_microphone_permission")
      .then((status) => setHasMicrophone(status === "granted"))
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

  return (
    <div style={{ maxWidth: "580px", margin: "0 auto" }}>
      {/* Alerts */}
      {hasModel === false && (
        <AlertBanner
          icon={<Download style={{ width: "16px", height: "16px" }} />}
          title="Download a model"
          description="You need a Whisper model to transcribe. Base (142 MB) is recommended."
          action={<button className="btn-primary" style={{ height: "32px", fontSize: "12px", padding: "0 14px" }} onClick={() => navigate("/models")}>Go to Models</button>}
        />
      )}

      {hasAccessibility === false && (
        <AlertBanner
          icon={<AlertTriangle style={{ width: "16px", height: "16px" }} />}
          title="Accessibility required"
          description="Grant access for global shortcuts and text injection."
          action={
            <button className="btn-primary" style={{ height: "32px", fontSize: "12px", padding: "0 14px" }} onClick={openAccessibility}>
              <ExternalLink style={{ width: "12px", height: "12px" }} />
              Grant Access
            </button>
          }
        />
      )}

      {/* Hero section */}
      <div style={{ textAlign: "center", padding: "48px 0 40px" }}>
        <RecordButton />
        <div style={{ marginTop: "20px" }}>
          <AudioVisualizer />
        </div>
      </div>

      {/* Error */}
      {error && (
        <div
          className="card"
          style={{
            padding: "12px 16px",
            borderColor: "var(--color-danger)",
            marginBottom: "20px",
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
          }}
        >
          <span style={{ fontSize: "13px", color: "var(--color-danger)" }}>{error}</span>
          <button
            onClick={clearError}
            style={{ fontSize: "12px", color: "var(--color-text-tertiary)", cursor: "pointer", background: "none", border: "none" }}
          >
            Dismiss
          </button>
        </div>
      )}

      {/* Last transcription */}
      {lastTranscription && (
        <div className="card" style={{ marginBottom: "24px", borderColor: "var(--color-accent)" }}>
          <div style={{ fontSize: "11px", fontWeight: 500, color: "var(--color-accent)", marginBottom: "6px", textTransform: "uppercase", letterSpacing: "0.05em" }}>
            Last transcription
          </div>
          <p className="select-text" style={{ fontSize: "14px", lineHeight: "1.6" }}>{lastTranscription}</p>
        </div>
      )}

      {/* Recent history */}
      {records.length > 0 && (
        <div>
          <h2 style={{ fontSize: "12px", fontWeight: 500, color: "var(--color-text-tertiary)", marginBottom: "12px", textTransform: "uppercase", letterSpacing: "0.05em" }}>
            Recent
          </h2>
          <div style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
            {records.slice(0, 5).map((record) => (
              <TranscriptionCard
                key={record.id}
                record={record}
                onCopy={handleCopy}
                onResend={handleResend}
              />
            ))}
          </div>
        </div>
      )}
    </div>
  );
}

function AlertBanner({ icon, title, description, action }: {
  icon: React.ReactNode;
  title: string;
  description: string;
  action: React.ReactNode;
}) {
  return (
    <div
      className="card"
      style={{ padding: "16px 20px", marginBottom: "16px", display: "flex", alignItems: "flex-start", gap: "12px" }}
    >
      <div style={{ color: "var(--color-warning)", marginTop: "2px" }}>{icon}</div>
      <div style={{ flex: 1 }}>
        <div style={{ fontSize: "13px", fontWeight: 550, marginBottom: "4px" }}>{title}</div>
        <div style={{ fontSize: "12px", color: "var(--color-text-secondary)", marginBottom: "10px" }}>{description}</div>
        {action}
      </div>
    </div>
  );
}
