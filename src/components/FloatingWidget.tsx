import { Mic, Square, Loader2 } from "lucide-react";
import clsx from "clsx";
import { useRecordingStore } from "../stores/recordingStore";
import { useEffect, useRef } from "react";

export default function FloatingWidget() {
  const { state, level, startRecording, transcribeAndInject, updateLevel } =
    useRecordingStore();
  const frameRef = useRef<number | null>(null);

  useEffect(() => {
    if (state !== "recording") {
      if (frameRef.current) cancelAnimationFrame(frameRef.current);
      return;
    }
    const tick = () => {
      updateLevel();
      frameRef.current = requestAnimationFrame(tick);
    };
    frameRef.current = requestAnimationFrame(tick);
    return () => {
      if (frameRef.current) cancelAnimationFrame(frameRef.current);
    };
  }, [state, updateLevel]);

  const handleClick = async () => {
    if (state === "idle") {
      await startRecording();
    } else if (state === "recording") {
      await transcribeAndInject();
    }
  };

  return (
    <div
      className={clsx(
        "flex items-center gap-2 rounded-full px-3 py-1.5 transition-all duration-200",
        "bg-white/90 dark:bg-surface-900/90 backdrop-blur-md",
        "border border-surface-200/50 dark:border-surface-700/50",
        "shadow-lg cursor-pointer select-none drag-region",
        state === "recording" && "px-4 border-red-300 dark:border-red-700"
      )}
      onDoubleClick={handleClick}
    >
      <button
        onClick={handleClick}
        className={clsx(
          "no-drag w-7 h-7 rounded-full flex items-center justify-center transition-colors",
          state === "idle" && "bg-accent-600 hover:bg-accent-700",
          state === "recording" && "bg-red-500",
          state === "processing" && "bg-surface-300 dark:bg-surface-700"
        )}
      >
        {state === "idle" && <Mic className="w-3.5 h-3.5 text-white" />}
        {state === "recording" && (
          <Square className="w-3 h-3 text-white fill-white" />
        )}
        {state === "processing" && (
          <Loader2 className="w-3.5 h-3.5 text-surface-500 animate-spin" />
        )}
      </button>

      {state === "recording" && (
        <div className="flex items-center gap-0.5 h-5">
          {Array.from({ length: 5 }).map((_, i) => (
            <div
              key={i}
              className="w-0.5 bg-red-400 rounded-full transition-all duration-75"
              style={{
                height: `${Math.max(4, level * 20 * (0.5 + Math.random() * 0.5))}px`,
              }}
            />
          ))}
        </div>
      )}

      {state === "idle" && (
        <span className="no-drag text-[11px] font-medium text-surface-500">
          VoxForge
        </span>
      )}

      {state === "processing" && (
        <span className="no-drag text-[11px] text-surface-500">...</span>
      )}
    </div>
  );
}
