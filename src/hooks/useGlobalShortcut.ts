import { useEffect, useRef } from "react";
import { useRecordingStore } from "../stores/recordingStore";

export function useGlobalShortcut() {
  const storeRef = useRef(useRecordingStore.getState());

  useEffect(() => {
    return useRecordingStore.subscribe((state) => {
      storeRef.current = state;
    });
  }, []);

  useEffect(() => {
    let cleanup: (() => void) | null = null;

    const setup = async () => {
      try {
        const { register, unregister } = await import(
          "@tauri-apps/plugin-global-shortcut"
        );

        await register("CommandOrControl+Shift+Space", async (event) => {
          const { state, startRecording, transcribeAndInject } = storeRef.current;
          if (event.state === "Pressed") {
            if (state === "idle") {
              await startRecording();
            }
          } else if (event.state === "Released") {
            if (storeRef.current.state === "recording") {
              await transcribeAndInject();
            }
          }
        });

        cleanup = () => {
          unregister("CommandOrControl+Shift+Space").catch(() => {});
        };
      } catch (e) {
        console.warn("Failed to register global shortcut:", e);
      }
    };

    setup();

    return () => {
      cleanup?.();
    };
  }, []);
}
