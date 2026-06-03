import { useEffect, useState } from "react";
import { BarChart3, Clock, Hash, Zap } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";

interface UsageStats {
  total_words: number;
  total_duration_ms: number;
  total_sessions: number;
  total_transcriptions: number;
  average_wpm: number;
}

interface DailyStats {
  date: string;
  total_words: number;
  total_duration_ms: number;
  session_count: number;
  transcription_count: number;
}

export default function Stats() {
  const [stats, setStats] = useState<UsageStats | null>(null);
  const [daily, setDaily] = useState<DailyStats[]>([]);

  useEffect(() => {
    const fetchStats = async () => {
      try {
        const overall = await invoke<UsageStats>("get_overall_stats");
        setStats(overall);
        const dailyStats = await invoke<DailyStats[]>("get_daily_stats", { days: 30 });
        setDaily(dailyStats);
      } catch {
        // ignore
      }
    };
    fetchStats();
  }, []);

  if (!stats) {
    return <div className="text-center py-12 text-surface-500">Loading...</div>;
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-xl font-bold">Usage Analytics</h1>
        <p className="text-sm text-surface-500 mt-1">
          Track your voice dictation productivity over time.
        </p>
      </div>

      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
        <StatCard
          icon={<Hash className="w-5 h-5" />}
          label="Total Words"
          value={stats.total_words.toLocaleString()}
        />
        <StatCard
          icon={<Clock className="w-5 h-5" />}
          label="Speaking Time"
          value={formatDuration(stats.total_duration_ms)}
        />
        <StatCard
          icon={<BarChart3 className="w-5 h-5" />}
          label="Sessions"
          value={stats.total_sessions.toLocaleString()}
        />
        <StatCard
          icon={<Zap className="w-5 h-5" />}
          label="Avg WPM"
          value={stats.average_wpm.toFixed(0)}
        />
      </div>

      <div className="card">
        <h3 className="text-sm font-semibold mb-4">Daily Activity (Last 30 days)</h3>
        {daily.length === 0 ? (
          <p className="text-sm text-surface-500 text-center py-8">
            No activity yet. Start dictating to see your stats.
          </p>
        ) : (
          <div className="space-y-2">
            <div className="flex items-end gap-1 h-32">
              {daily.reverse().map((day) => {
                const maxWords = Math.max(...daily.map((d) => d.total_words), 1);
                const height = (day.total_words / maxWords) * 100;
                return (
                  <div
                    key={day.date}
                    className="flex-1 group relative"
                    title={`${day.date}: ${day.total_words} words`}
                  >
                    <div
                      className="w-full bg-accent-400 dark:bg-accent-600 rounded-t opacity-80 hover:opacity-100 transition-opacity"
                      style={{ height: `${Math.max(height, 2)}%` }}
                    />
                  </div>
                );
              })}
            </div>
            <div className="flex justify-between text-[10px] text-surface-400">
              <span>30 days ago</span>
              <span>Today</span>
            </div>
          </div>
        )}
      </div>

      <div className="card">
        <h3 className="text-sm font-semibold mb-3">Summary</h3>
        <div className="text-sm text-surface-600 dark:text-surface-400 space-y-1">
          <p>Total transcriptions: {stats.total_transcriptions.toLocaleString()}</p>
          <p>
            Average words per session:{" "}
            {stats.total_sessions > 0
              ? Math.round(stats.total_words / stats.total_sessions)
              : 0}
          </p>
        </div>
      </div>
    </div>
  );
}

function StatCard({
  icon,
  label,
  value,
}: {
  icon: React.ReactNode;
  label: string;
  value: string;
}) {
  return (
    <div className="card">
      <div className="flex items-center gap-2 text-surface-500 mb-2">
        {icon}
        <span className="text-xs font-medium">{label}</span>
      </div>
      <div className="text-2xl font-bold">{value}</div>
    </div>
  );
}

function formatDuration(ms: number): string {
  const minutes = Math.floor(ms / 60000);
  if (minutes < 60) return `${minutes}m`;
  const hours = Math.floor(minutes / 60);
  return `${hours}h ${minutes % 60}m`;
}
