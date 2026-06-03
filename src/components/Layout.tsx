import { Outlet, NavLink } from "react-router-dom";
import {
  Mic,
  Clock,
  BookOpen,
  Download,
  Settings,
  BarChart3,
  Shield,
} from "lucide-react";
import clsx from "clsx";

const navItems = [
  { to: "/", icon: Mic, label: "Dashboard" },
  { to: "/history", icon: Clock, label: "History" },
  { to: "/dictionary", icon: BookOpen, label: "Dictionary" },
  { to: "/models", icon: Download, label: "Models" },
  { to: "/stats", icon: BarChart3, label: "Analytics" },
  { to: "/privacy", icon: Shield, label: "Privacy" },
  { to: "/settings", icon: Settings, label: "Settings" },
];

export default function Layout() {
  return (
    <div className="flex h-screen">
      <aside className="w-56 border-r border-surface-200 dark:border-surface-800 bg-surface-50 dark:bg-surface-950 flex flex-col">
        <div className="h-12 flex items-center px-4 drag-region border-b border-surface-200 dark:border-surface-800">
          <div className="no-drag flex items-center gap-2">
            <div className="w-6 h-6 rounded-md bg-accent-600 flex items-center justify-center">
              <Mic className="w-3.5 h-3.5 text-white" />
            </div>
            <span className="font-semibold text-sm tracking-tight">
              VoxForge
            </span>
          </div>
        </div>

        <nav className="flex-1 p-3 space-y-1">
          {navItems.map(({ to, icon: Icon, label }) => (
            <NavLink
              key={to}
              to={to}
              end={to === "/"}
              className={({ isActive }) =>
                clsx("sidebar-item", isActive && "active")
              }
            >
              <Icon className="w-4 h-4" />
              <span>{label}</span>
            </NavLink>
          ))}
        </nav>

        <div className="p-3 border-t border-surface-200 dark:border-surface-800">
          <div className="text-xs text-surface-500 text-center">
            VoxForge v0.1.0
          </div>
        </div>
      </aside>

      <main className="flex-1 flex flex-col overflow-hidden">
        <div className="h-12 drag-region border-b border-surface-200 dark:border-surface-800 flex items-center px-4">
          <div className="no-drag ml-auto flex items-center gap-2">
            <StatusBadge />
          </div>
        </div>
        <div className="flex-1 overflow-y-auto p-6">
          <Outlet />
        </div>
      </main>
    </div>
  );
}

function StatusBadge() {
  return (
    <div className="flex items-center gap-2 text-xs">
      <div className="w-2 h-2 rounded-full bg-green-500" />
      <span className="text-surface-600 dark:text-surface-400">Ready</span>
    </div>
  );
}
