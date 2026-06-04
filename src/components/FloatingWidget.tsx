import { Mic, Square, Loader2 } from "lucide-react";
import clsx from "clsx";
import { useRecordingStore } from "../stores/recordingStore";
import { useEffect, useRef } from "react";

export default function FloatingWidget() {
  const { state, level, updateLevel } = useRecordingStore();
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


  return (
    <div
      className={clsx(
        "flex items-center gap-1.5 rounded-full px-2.5 py-1 transition-all duration-200",
        "bg-white/90 dark:bg-surface-900/90 backdrop-blur-md",
        "border border-surface-200/50 dark:border-surface-700/50",
        "shadow-lg select-none",
        state === "recording" && "px-3 border-red-300 dark:border-red-700"
      )}
    >
      <div
        className={clsx(
          "w-5 h-5 rounded-full flex items-center justify-center",
          state === "idle" && "bg-accent-600",
          state === "recording" && "bg-red-500",
          state === "processing" && "bg-surface-300 dark:bg-surface-700"
        )}
      >
        {state === "idle" && <Mic className="w-2.5 h-2.5 text-white" />}
        {state === "recording" && (
          <Square className="w-2 h-2 text-white fill-white" />
        )}
        {state === "processing" && (
          <Loader2 className="w-2.5 h-2.5 text-surface-500 animate-spin" />
        )}
      </div>

      {state === "recording" && (
        <div className="flex items-center gap-px h-4">
          {Array.from({ length: 7 }).map((_, i) => (
            <div
              key={i}
              className="w-[2px] bg-red-400 rounded-full transition-all duration-75"
              style={{
                height: `${Math.max(3, level * 16 * (0.4 + Math.random() * 0.6))}px`,
              }}
            />
          ))}
        </div>
      )}

      {state === "processing" && (
        <span className="text-[10px] text-surface-500">...</span>
      )}
    </div>
  );
}
