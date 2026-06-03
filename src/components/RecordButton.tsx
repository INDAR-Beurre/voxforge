import { Mic, Square, Loader2 } from "lucide-react";
import clsx from "clsx";
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
    <div className="flex flex-col items-center gap-4">
      <button
        onClick={handleClick}
        disabled={state === "processing"}
        className={clsx(
          "relative w-20 h-20 rounded-full flex items-center justify-center",
          "transition-all duration-200 focus:outline-none",
          state === "idle" &&
            "bg-accent-600 hover:bg-accent-700 hover:scale-105 shadow-lg shadow-accent-600/25",
          state === "recording" &&
            "bg-red-500 hover:bg-red-600 scale-110 shadow-lg shadow-red-500/30",
          state === "processing" &&
            "bg-surface-300 dark:bg-surface-700 cursor-not-allowed"
        )}
      >
        {state === "recording" && (
          <div className="absolute inset-0 rounded-full border-2 border-red-400 animate-pulse-ring" />
        )}
        <RecordIcon state={state} />
      </button>

      <span className="text-sm font-medium text-surface-600 dark:text-surface-400">
        {state === "idle" && "Press to record"}
        {state === "recording" && "Recording... tap to stop"}
        {state === "processing" && "Transcribing..."}
      </span>
    </div>
  );
}

function RecordIcon({ state }: { state: RecordingState }) {
  if (state === "processing") {
    return <Loader2 className="w-8 h-8 text-surface-500 animate-spin" />;
  }
  if (state === "recording") {
    return <Square className="w-6 h-6 text-white fill-white" />;
  }
  return <Mic className="w-8 h-8 text-white" />;
}
