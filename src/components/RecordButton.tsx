import { Mic, Square, Loader2 } from "lucide-react";
import { useRecordingStore, RecordingState } from "../stores/recordingStore";

export default function RecordButton() {
  const { state, startRecording, transcribeAndInject } = useRecordingStore();

  const handleClick = async () => {
    if (state === "idle") {
      await startRecording();
    } else if (state === "recording") {
      await transcribeAndInject();
    }
  };

  return (
    <div className="flex flex-col items-center gap-5">
      <button
        onClick={handleClick}
        disabled={state === "processing"}
        style={{
          width: "72px",
          height: "72px",
          borderRadius: "50%",
          border: "none",
          cursor: state === "processing" ? "not-allowed" : "pointer",
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
          transition: "transform 0.2s cubic-bezier(0.87, 0, 0.13, 1), box-shadow 0.2s ease",
          transform: state === "recording" ? "scale(1.08)" : "scale(1)",
          background:
            state === "idle"
              ? "var(--color-text)"
              : state === "recording"
              ? "#ff453a"
              : "var(--color-fill-secondary)",
          boxShadow:
            state === "idle"
              ? "0 4px 20px rgba(0,0,0,0.12)"
              : state === "recording"
              ? "0 4px 24px rgba(255,69,58,0.3)"
              : "none",
        }}
      >
        <RecordIcon state={state} />
      </button>

      <span
        style={{
          fontSize: "13px",
          fontWeight: 450,
          color: "var(--color-text-secondary)",
          letterSpacing: "-0.01em",
        }}
      >
        {state === "idle" && "Ctrl+Shift+S to record"}
        {state === "recording" && "Listening..."}
        {state === "processing" && "Transcribing..."}
      </span>
    </div>
  );
}

function RecordIcon({ state }: { state: RecordingState }) {
  const color = state === "idle" || state === "recording" ? "var(--color-bg)" : "var(--color-text-tertiary)";
  if (state === "processing") {
    return <Loader2 style={{ width: "24px", height: "24px", color }} className="animate-spin" />;
  }
  if (state === "recording") {
    return <Square style={{ width: "20px", height: "20px", color, fill: color }} />;
  }
  return <Mic style={{ width: "26px", height: "26px", color }} />;
}
