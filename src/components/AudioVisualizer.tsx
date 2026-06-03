import { useEffect, useRef } from "react";
import { useRecordingStore } from "../stores/recordingStore";

export default function AudioVisualizer() {
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

  if (state !== "recording") return null;

  const bars = 12;

  return (
    <div className="flex items-center justify-center gap-0.5 h-8">
      {Array.from({ length: bars }).map((_, i) => {
        const barLevel =
          Math.max(0.15, level * (0.5 + Math.random() * 0.5));
        return (
          <div
            key={i}
            className="w-1 bg-accent-500 rounded-full transition-all duration-75"
            style={{
              height: `${barLevel * 32}px`,
              opacity: 0.6 + barLevel * 0.4,
            }}
          />
        );
      })}
    </div>
  );
}
