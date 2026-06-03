import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { AppSettings } from "../stores/settingsStore";
import { useSettingsStore } from "../stores/settingsStore";
import { useThemeStore } from "../stores/themeStore";

export default function Settings() {
  const { settings, fetchSettings, updateSetting } =
    useSettingsStore();
  const { theme, setTheme } = useThemeStore();
  const [devices, setDevices] = useState<string[]>([]);

  useEffect(() => {
    fetchSettings();
    invoke<string[]>("list_audio_devices").then(setDevices).catch(() => {});
  }, [fetchSettings]);

  if (!settings) return <div className="text-center py-12 text-surface-500">Loading...</div>;

  const handleChange = (key: keyof AppSettings, value: string | boolean | number) => {
    updateSetting(key, String(value));
  };

  return (
    <div className="space-y-8 max-w-2xl">
      <h1 className="text-xl font-bold">Settings</h1>

      <Section title="Recording">
        <SettingRow label="Hotkey Mode">
          <select
            value={settings.hotkey_mode}
            onChange={(e) => handleChange("hotkey_mode", e.target.value)}
            className="input w-44"
          >
            <option value="push_to_talk">Push to Talk</option>
            <option value="toggle">Toggle</option>
          </select>
        </SettingRow>

        <SettingRow label="Push-to-Talk Shortcut">
          <input
            type="text"
            value={settings.push_to_talk_key}
            onChange={(e) => handleChange("push_to_talk_key", e.target.value)}
            className="input w-56"
          />
        </SettingRow>

        <SettingRow label="Toggle Shortcut">
          <input
            type="text"
            value={settings.toggle_key}
            onChange={(e) => handleChange("toggle_key", e.target.value)}
            className="input w-56"
          />
        </SettingRow>

        <SettingRow label="Input Device">
          <select
            onChange={(e) => invoke("set_audio_device", { name: e.target.value })}
            className="input w-56"
          >
            <option value="">System Default</option>
            {devices.map((d) => (
              <option key={d} value={d}>{d}</option>
            ))}
          </select>
        </SettingRow>

        <SettingRow label="Silence Timeout">
          <div className="flex items-center gap-2">
            <input
              type="range"
              min={500}
              max={5000}
              step={100}
              value={settings.silence_timeout_ms}
              onChange={(e) => handleChange("silence_timeout_ms", Number(e.target.value))}
              className="w-32"
            />
            <span className="text-xs text-surface-500 w-12">
              {settings.silence_timeout_ms}ms
            </span>
          </div>
        </SettingRow>

        <SettingRow label="Start/Stop Sounds">
          <Toggle
            checked={settings.play_sounds}
            onChange={(v) => handleChange("play_sounds", v)}
          />
        </SettingRow>
      </Section>

      <Section title="Transcription">
        <SettingRow label="Provider">
          <select
            value={settings.transcription_provider}
            onChange={(e) => handleChange("transcription_provider", e.target.value)}
            className="input w-44"
          >
            <option value="local">Local (Whisper)</option>
            <option value="cloud">Cloud</option>
          </select>
        </SettingRow>

        <SettingRow label="Language">
          <select
            value={settings.language}
            onChange={(e) => handleChange("language", e.target.value)}
            className="input w-44"
          >
            <option value="en">English</option>
            <option value="es">Spanish</option>
            <option value="fr">French</option>
            <option value="de">German</option>
            <option value="ja">Japanese</option>
            <option value="zh">Chinese</option>
            <option value="ko">Korean</option>
            <option value="pt">Portuguese</option>
            <option value="auto">Auto-detect</option>
          </select>
        </SettingRow>

        {settings.transcription_provider === "cloud" && (
          <>
            <SettingRow label="Cloud Provider Name">
              <input
                type="text"
                value={settings.cloud_provider_name}
                onChange={(e) => handleChange("cloud_provider_name", e.target.value)}
                placeholder="e.g., OpenAI"
                className="input w-56"
              />
            </SettingRow>
            <SettingRow label="API Endpoint">
              <input
                type="text"
                value={settings.cloud_provider_url}
                onChange={(e) => handleChange("cloud_provider_url", e.target.value)}
                placeholder="https://api.example.com/v1/audio/transcriptions"
                className="input w-full"
              />
            </SettingRow>
          </>
        )}
      </Section>

      <Section title="Text Injection">
        <SettingRow label="Strategy">
          <select
            value={settings.injection_strategy}
            onChange={(e) => handleChange("injection_strategy", e.target.value)}
            className="input w-44"
          >
            <option value="clipboard_paste">Clipboard + Paste</option>
            <option value="keyboard_simulation">Keyboard Simulation</option>
          </select>
        </SettingRow>

        <SettingRow label="Preserve Clipboard">
          <Toggle
            checked={settings.preserve_clipboard}
            onChange={(v) => handleChange("preserve_clipboard", v)}
          />
        </SettingRow>
      </Section>

      <Section title="Appearance">
        <SettingRow label="Theme">
          <select
            value={theme}
            onChange={(e) => setTheme(e.target.value as "light" | "dark" | "system")}
            className="input w-44"
          >
            <option value="system">System</option>
            <option value="light">Light</option>
            <option value="dark">Dark</option>
          </select>
        </SettingRow>

        <SettingRow label="Compact Mode">
          <Toggle
            checked={settings.compact_mode}
            onChange={(v) => handleChange("compact_mode", v)}
          />
        </SettingRow>

        <SettingRow label="Translucency">
          <Toggle
            checked={settings.translucency}
            onChange={(v) => handleChange("translucency", v)}
          />
        </SettingRow>

        <SettingRow label="Reduced Motion">
          <Toggle
            checked={settings.reduced_motion}
            onChange={(v) => handleChange("reduced_motion", v)}
          />
        </SettingRow>

        <SettingRow label="Sound Cues">
          <Toggle
            checked={settings.sound_cues}
            onChange={(v) => handleChange("sound_cues", v)}
          />
        </SettingRow>
      </Section>

      <Section title="Permissions">
        <SettingRow label="Accessibility">
          <button
            onClick={() => invoke("open_accessibility_settings")}
            className="btn-secondary text-xs py-1.5 px-3"
          >
            Open Settings
          </button>
        </SettingRow>
        <SettingRow label="Microphone">
          <button
            onClick={() => invoke("open_microphone_settings")}
            className="btn-secondary text-xs py-1.5 px-3"
          >
            Open Settings
          </button>
        </SettingRow>
      </Section>
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="space-y-3">
      <h2 className="text-sm font-semibold text-surface-700 dark:text-surface-300 border-b border-surface-200 dark:border-surface-800 pb-2">
        {title}
      </h2>
      <div className="space-y-4">{children}</div>
    </div>
  );
}

function SettingRow({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div className="flex items-center justify-between gap-4">
      <label className="text-sm text-surface-700 dark:text-surface-300">{label}</label>
      {children}
    </div>
  );
}

function Toggle({ checked, onChange }: { checked: boolean; onChange: (v: boolean) => void }) {
  return (
    <button
      onClick={() => onChange(!checked)}
      className={`relative w-10 h-5 rounded-full transition-colors ${
        checked ? "bg-accent-600" : "bg-surface-300 dark:bg-surface-700"
      }`}
    >
      <div
        className={`absolute top-0.5 w-4 h-4 rounded-full bg-white shadow transition-transform ${
          checked ? "translate-x-5" : "translate-x-0.5"
        }`}
      />
    </button>
  );
}
