import { useEffect } from "react";
import { useRecordingStore } from "../stores/recordingStore";

export function useGlobalShortcut() {
  const { state, startRecording, transcribeAndInject } = useRecordingStore();

  useEffect(() => {
    let registered = false;

    const setup = async () => {
      try {
        const { register } = await import(
          "@tauri-apps/plugin-global-shortcut"
        );

        await register("CommandOrControl+Shift+Space", async (event) => {
          if (event.state === "Pressed") {
            if (state === "idle") {
              await startRecording();
            }
          } else if (event.state === "Released") {
            if (state === "recording") {
              await transcribeAndInject();
            }
          }
        });

        registered = true;
      } catch (e) {
        console.warn("Failed to register global shortcut:", e);
      }
    };

    setup();

    return () => {
      if (registered) {
        import("@tauri-apps/plugin-global-shortcut").then(({ unregister }) => {
          unregister("CommandOrControl+Shift+Space").catch(() => {});
        });
      }
    };
  }, [state, startRecording, transcribeAndInject]);
}
