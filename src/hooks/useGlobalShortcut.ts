import { useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
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
    let unlisten1: (() => void) | null = null;
    let unlisten2: (() => void) | null = null;

    const setup = async () => {
      // Start the fn key monitor on the Rust side
      try {
        await invoke("start_fn_key_monitor");
      } catch (e) {
        console.warn("Failed to start fn key monitor:", e);
      }

      // Listen for fn key down → start recording + show widget
      unlisten1 = await listen("fn-key-down", async () => {
        const { state, startRecording } = storeRef.current;
        if (state === "idle") {
          await startRecording();
          showWidget();
        }
      });

      // Listen for fn key up → stop recording + transcribe + hide widget
      unlisten2 = await listen("fn-key-up", async () => {
        const { state, transcribeAndInject } = storeRef.current;
        if (state === "recording") {
          await transcribeAndInject();
          // Give a small delay for the user to see "processing" state
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
    const current = getCurrentWebviewWindow();
    if (current.label === "widget") return;

    const widget = await WebviewWindow.getByLabel("widget");
    if (widget) {
      await widget.show();
      await widget.setFocus();
    }
  } catch (e) {
    console.warn("Failed to show widget:", e);
  }
}

async function hideWidget() {
  try {
    const current = getCurrentWebviewWindow();
    if (current.label === "widget") return;

    const widget = await WebviewWindow.getByLabel("widget");
    if (widget) {
      await widget.hide();
    }
  } catch (e) {
    console.warn("Failed to hide widget:", e);
  }
}
