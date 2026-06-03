import { Shield, Lock, Wifi, WifiOff, HardDrive, Eye } from "lucide-react";

export default function Privacy() {
  return (
    <div className="space-y-6 max-w-2xl">
      <div>
        <h1 className="text-xl font-bold">Privacy Center</h1>
        <p className="text-sm text-surface-500 mt-1">
          Understand exactly what stays on your device and what doesn't.
        </p>
      </div>

      <div className="card border-green-200 dark:border-green-800 bg-green-50/50 dark:bg-green-900/10">
        <div className="flex items-start gap-3">
          <Shield className="w-5 h-5 text-green-600 dark:text-green-400 mt-0.5" />
          <div>
            <h3 className="font-semibold text-sm text-green-800 dark:text-green-300">
              Privacy-First Design
            </h3>
            <p className="text-sm text-green-700 dark:text-green-400 mt-1">
              VoxForge is designed to keep your voice data private by default.
              Local transcription means your audio never leaves your computer.
            </p>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        <PrivacyCard
          icon={<HardDrive className="w-5 h-5" />}
          title="Local Transcription"
          description="Audio is processed entirely on-device using Whisper. No data is sent to any server."
          status="Always Local"
          statusColor="green"
        />
        <PrivacyCard
          icon={<Lock className="w-5 h-5" />}
          title="History Storage"
          description="Transcription history is stored in a local SQLite database on your Mac. Only you can access it."
          status="On Device"
          statusColor="green"
        />
        <PrivacyCard
          icon={<WifiOff className="w-5 h-5" />}
          title="Offline Mode"
          description="VoxForge works completely offline when using local models. No internet required."
          status="No Network"
          statusColor="green"
        />
        <PrivacyCard
          icon={<Wifi className="w-5 h-5" />}
          title="Cloud Providers"
          description="When cloud transcription is enabled, audio is sent to your configured provider. This is opt-in only."
          status="Opt-in"
          statusColor="yellow"
        />
        <PrivacyCard
          icon={<Eye className="w-5 h-5" />}
          title="Analytics"
          description="Usage statistics (word count, duration) are stored locally. Nothing is sent externally."
          status="Local Only"
          statusColor="green"
        />
        <PrivacyCard
          icon={<Lock className="w-5 h-5" />}
          title="API Keys"
          description="If you add a cloud provider API key, it's stored in the local encrypted keychain."
          status="Encrypted"
          statusColor="green"
        />
      </div>

      <div className="card">
        <h3 className="font-semibold text-sm mb-3">Data Retention</h3>
        <ul className="space-y-2 text-sm text-surface-600 dark:text-surface-400">
          <li className="flex items-start gap-2">
            <span className="text-green-500 mt-0.5">•</span>
            Audio recordings are processed in memory and immediately discarded after transcription.
          </li>
          <li className="flex items-start gap-2">
            <span className="text-green-500 mt-0.5">•</span>
            Transcription text is saved to local history (you can delete anytime).
          </li>
          <li className="flex items-start gap-2">
            <span className="text-green-500 mt-0.5">•</span>
            No audio files are ever written to disk.
          </li>
          <li className="flex items-start gap-2">
            <span className="text-green-500 mt-0.5">•</span>
            No telemetry, no tracking, no user accounts.
          </li>
        </ul>
      </div>

      <div className="card">
        <h3 className="font-semibold text-sm mb-3">Permissions Explained</h3>
        <ul className="space-y-3 text-sm text-surface-600 dark:text-surface-400">
          <li>
            <strong className="text-surface-800 dark:text-surface-200">Microphone:</strong>{" "}
            Required to capture your voice for transcription.
          </li>
          <li>
            <strong className="text-surface-800 dark:text-surface-200">Accessibility:</strong>{" "}
            Required to simulate paste (Cmd+V) into the focused app and register global shortcuts.
          </li>
          <li>
            <strong className="text-surface-800 dark:text-surface-200">Network (optional):</strong>{" "}
            Only used if you enable cloud transcription or download new models.
          </li>
        </ul>
      </div>
    </div>
  );
}

function PrivacyCard({
  icon,
  title,
  description,
  status,
  statusColor,
}: {
  icon: React.ReactNode;
  title: string;
  description: string;
  status: string;
  statusColor: "green" | "yellow";
}) {
  return (
    <div className="card">
      <div className="flex items-start justify-between mb-2">
        <div className="text-surface-600 dark:text-surface-400">{icon}</div>
        <span
          className={`text-[10px] font-medium px-1.5 py-0.5 rounded ${
            statusColor === "green"
              ? "bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300"
              : "bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-300"
          }`}
        >
          {status}
        </span>
      </div>
      <h4 className="text-sm font-semibold">{title}</h4>
      <p className="text-xs text-surface-500 mt-1">{description}</p>
    </div>
  );
}
