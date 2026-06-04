import { useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow, WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { useRecordingStore } from "../stores/recordingStore";

export function useGlobalShortcut() {
  const storeRef = useRef(useRecordingStore.getState());

  useEffect(() => {
    return useRecordingStore.subscribe((state) => {
      storeRef.current = state;
    });
  }, []);

  useEffect(() => {
    const current = getCurrentWebviewWindow();
    if (current.label !== "main") return;

    let cleanup: (() => void) | null = null;

    const setup = async () => {
      try {
        const { register, unregister } = await import(
          "@tauri-apps/plugin-global-shortcut"
        );

        await register("Control+Shift+S", async (event) => {
          if (event.state === "Pressed") {
            const { state, startRecording } = storeRef.current;
            if (state === "idle") {
              await startRecording();
              showWidget();
            }
          } else if (event.state === "Released") {
            const { state, transcribeAndInject } = storeRef.current;
            if (state === "recording") {
              await transcribeAndInject();
              setTimeout(hideWidget, 1500);
            }
          }
        });

        cleanup = () => {
          unregister("Control+Shift+S").catch(() => {});
        };
      } catch (e) {
        console.warn("Failed to register global shortcut:", e);
      }

      // Also try fn key monitor (works if accessibility is granted)
      try {
        await invoke("start_fn_key_monitor");
      } catch (_) {}
    };

    setup();

    return () => {
      cleanup?.();
    };
  }, []);

  // Also listen for fn key events as a secondary trigger
  useEffect(() => {
    const current = getCurrentWebviewWindow();
    if (current.label !== "main") return;

    let unlisten1: (() => void) | null = null;
    let unlisten2: (() => void) | null = null;

    const setup = async () => {
      const { listen } = await import("@tauri-apps/api/event");

      unlisten1 = await listen("fn-key-down", async () => {
        const { state, startRecording } = storeRef.current;
        if (state === "idle") {
          await startRecording();
          showWidget();
        }
      });

      unlisten2 = await listen("fn-key-up", async () => {
        const { state, transcribeAndInject } = storeRef.current;
        if (state === "recording") {
          await transcribeAndInject();
          setTimeout(hideWidget, 1500);
        }
      });
    };

    setup();

    return () => {
      unlisten1?.();
      unlisten2?.();
    };
  }, []);
}

async function showWidget() {
  try {
    const widget = await WebviewWindow.getByLabel("widget");
    if (widget) {
      await widget.show();
    }
  } catch (e) {
    console.warn("Failed to show widget:", e);
  }
}

async function hideWidget() {
  try {
    const widget = await WebviewWindow.getByLabel("widget");
    if (widget) {
      await widget.hide();
    }
  } catch (e) {
    console.warn("Failed to hide widget:", e);
  }
}
