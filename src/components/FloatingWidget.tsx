import { useRecordingStore } from "../stores/recordingStore";
import { useEffect, useRef } from "react";

export default function FloatingWidget() {
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
      const h = Math.max(3, level * 18 * (0.3 + Math.random() * 0.7));
      el.style.height = `${h}px`;
    }
  }, [level, state]);

  return (
    <div style={{
      width: "100%",
      height: "100%",
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
      background: "transparent",
    }}>
      <div style={{
        display: "flex",
        alignItems: "center",
        gap: "8px",
        padding: state === "recording" ? "6px 14px" : "6px 12px",
        borderRadius: "100px",
        background: "rgba(30, 30, 30, 0.85)",
        backdropFilter: "blur(20px)",
        WebkitBackdropFilter: "blur(20px)",
        boxShadow: "0 2px 12px rgba(0,0,0,0.3), inset 0 0.5px 0 rgba(255,255,255,0.08)",
        border: "0.5px solid rgba(255,255,255,0.1)",
        transition: "all 0.2s ease",
      }}>
        {/* Indicator dot */}
        <div style={{
          width: "8px",
          height: "8px",
          borderRadius: "50%",
          background: state === "recording" ? "#ff453a" : state === "processing" ? "#ff9f0a" : "#30d158",
          boxShadow: state === "recording" ? "0 0 6px rgba(255,69,58,0.6)" : "none",
          animation: state === "recording" ? "pulse 1.5s ease infinite" : "none",
        }} />

        {/* Waveform bars */}
        {state === "recording" && (
          <div ref={barsRef} style={{
            display: "flex",
            alignItems: "center",
            gap: "2px",
            height: "16px",
          }}>
            {Array.from({ length: 9 }).map((_, i) => (
              <div
                key={i}
                style={{
                  width: "2px",
                  height: "3px",
                  borderRadius: "1px",
                  background: "rgba(255,255,255,0.7)",
                  transition: "height 60ms ease-out",
                }}
              />
            ))}
          </div>
        )}

        {/* Processing spinner */}
        {state === "processing" && (
          <div style={{
            width: "12px",
            height: "12px",
            border: "1.5px solid rgba(255,255,255,0.2)",
            borderTop: "1.5px solid rgba(255,255,255,0.8)",
            borderRadius: "50%",
            animation: "spin 0.8s linear infinite",
          }} />
        )}
      </div>

      <style>{`
        @keyframes pulse {
          0%, 100% { opacity: 1; }
          50% { opacity: 0.5; }
        }
        @keyframes spin {
          from { transform: rotate(0deg); }
          to { transform: rotate(360deg); }
        }
      `}</style>
    </div>
  );
}
