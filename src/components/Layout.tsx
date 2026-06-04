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

const navItems = [
  { to: "/", icon: Mic, label: "Dictation" },
  { to: "/history", icon: Clock, label: "History" },
  { to: "/dictionary", icon: BookOpen, label: "Dictionary" },
  { to: "/models", icon: Download, label: "Models" },
  { to: "/stats", icon: BarChart3, label: "Analytics" },
  { to: "/privacy", icon: Shield, label: "Privacy" },
  { to: "/settings", icon: Settings, label: "Settings" },
];

export default function Layout() {
  return (
    <div className="flex h-screen" style={{ background: "var(--color-bg)" }}>
      {/* Sidebar */}
      <aside
        className="flex flex-col"
        style={{
          width: "200px",
          borderRight: "1px solid var(--color-border)",
          background: "var(--color-bg)",
        }}
      >
        {/* Title bar drag region */}
        <div
          className="drag-region flex items-center px-4"
          style={{ height: "52px", paddingTop: "6px" }}
        >
          <div className="no-drag flex items-center gap-2.5">
            <div
              style={{
                width: "24px",
                height: "24px",
                borderRadius: "7px",
                background: "var(--color-text)",
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
              }}
            >
              <Mic style={{ width: "13px", height: "13px", color: "var(--color-bg)" }} />
            </div>
            <span
              style={{
                fontSize: "14px",
                fontWeight: 600,
                letterSpacing: "-0.02em",
                color: "var(--color-text)",
              }}
            >
              VoxForge
            </span>
          </div>
        </div>

        {/* Nav */}
        <nav className="flex-1 px-3 pt-2" style={{ display: "flex", flexDirection: "column", gap: "2px" }}>
          {navItems.map(({ to, icon: Icon, label }) => (
            <NavLink
              key={to}
              to={to}
              end={to === "/"}
              className={({ isActive }) =>
                `sidebar-item ${isActive ? "active" : ""}`
              }
            >
              <Icon style={{ width: "16px", height: "16px" }} />
              <span>{label}</span>
            </NavLink>
          ))}
        </nav>

        {/* Footer */}
        <div className="px-4 pb-4">
          <div
            style={{
              fontSize: "11px",
              color: "var(--color-text-tertiary)",
              fontFamily: "SF Mono, Menlo, monospace",
              letterSpacing: "0.02em",
            }}
          >
            v0.2.0
          </div>
        </div>
      </aside>

      {/* Main content */}
      <main className="flex-1 flex flex-col overflow-hidden">
        {/* Top bar */}
        <div
          className="drag-region flex items-center justify-end px-5"
          style={{
            height: "52px",
            paddingTop: "6px",
            borderBottom: "1px solid var(--color-border)",
          }}
        >
          <div className="no-drag flex items-center gap-2">
            <div
              style={{
                width: "7px",
                height: "7px",
                borderRadius: "50%",
                background: "var(--color-success)",
              }}
            />
            <span
              style={{
                fontSize: "12px",
                color: "var(--color-text-tertiary)",
                fontWeight: 450,
              }}
            >
              Ready
            </span>
          </div>
        </div>

        {/* Content area */}
        <div className="flex-1 overflow-y-auto" style={{ padding: "32px" }}>
          <Outlet />
        </div>
      </main>
    </div>
  );
}
