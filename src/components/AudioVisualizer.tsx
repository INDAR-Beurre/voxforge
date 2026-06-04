import { useEffect, useRef } from "react";
import { useRecordingStore } from "../stores/recordingStore";

export default function AudioVisualizer() {
  const { state, level, updateLevel } = useRecordingStore();
  const frameRef = useRef<number | null>(null);
  const barsRef = useRef<HTMLDivElement>(null);

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

  useEffect(() => {
    if (!barsRef.current || state !== "recording") return;
    const bars = barsRef.current.children;
    for (let i = 0; i < bars.length; i++) {
      const el = bars[i] as HTMLElement;
      const h = Math.max(4, level * 28 * (0.3 + Math.random() * 0.7));
      el.style.height = `${h}px`;
    }
  }, [level, state]);

  if (state !== "recording") return null;

  return (
    <div
      ref={barsRef}
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        gap: "3px",
        height: "32px",
      }}
    >
      {Array.from({ length: 16 }).map((_, i) => (
        <div
          key={i}
          style={{
            width: "3px",
            height: "4px",
            borderRadius: "2px",
            background: "#ff453a",
            opacity: 0.7,
            transition: "height 60ms ease-out",
          }}
        />
      ))}
    </div>
  );
}
