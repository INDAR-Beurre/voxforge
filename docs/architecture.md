# Architecture Overview

## System Design

VoxForge follows a layered architecture with clear separation between the native system layer (Rust/Tauri), the transcription engine layer, and the presentation layer (React).

```
┌─────────────────────────────────────────────────────┐
│                   Frontend (React)                    │
│  Dashboard │ History │ Dictionary │ Models │ Settings │
├─────────────────────────────────────────────────────┤
│                 Tauri IPC Bridge                      │
├─────────────────────────────────────────────────────┤
│                  Rust Backend                         │
│  ┌──────────┐ ┌────────────┐ ┌──────────────────┐  │
│  │  Audio   │ │Transcription│ │  Text Injection  │  │
│  │ Capture  │ │   Engine   │ │  (Clipboard/Paste)│  │
│  └──────────┘ └────────────┘ └──────────────────┘  │
│  ┌──────────┐ ┌────────────┐ ┌──────────────────┐  │
│  │ Database │ │   Model    │ │  Post-Processing │  │
│  │ (SQLite) │ │  Manager   │ │     Pipeline     │  │
│  └──────────┘ └────────────┘ └──────────────────┘  │
├─────────────────────────────────────────────────────┤
│              macOS System Services                    │
│  CoreAudio │ Accessibility │ Clipboard │ Shortcuts   │
└─────────────────────────────────────────────────────┘
```

## Data Flow: Recording → Injection

1. User presses global hotkey (registered via tauri-plugin-global-shortcut)
2. Audio capture begins via cpal (CoreAudio backend on macOS)
3. Audio samples accumulate in a shared buffer (Arc<Mutex<Vec<f32>>>)
4. User releases hotkey (or presses stop)
5. Audio buffer is retrieved and resampled to 16kHz mono
6. Whisper model processes audio → text
7. Dictionary replacements are applied
8. Post-processing rules run (capitalization, cleanup)
9. Text is injected into focused app via:
   - Save current clipboard
   - Copy text to clipboard
   - Simulate Cmd+V via osascript/System Events
   - Restore previous clipboard
10. Transcription record saved to SQLite

## Module Responsibilities

### Audio (src/audio/)
- `capture.rs`: Microphone access via cpal, buffer management, level metering
- `processor.rs`: Resampling, normalization, silence detection, WAV encoding

### Transcription (src/transcription/)
- `engine.rs`: Trait definition for pluggable transcription backends
- `local.rs`: whisper-rs integration for on-device transcription
- `cloud.rs`: OpenAI-compatible API client for cloud transcription

### Injection (src/injection/)
- `text_injector.rs`: Clipboard-paste strategy, keyboard simulation fallback, focused app detection

### Database (src/database/)
- `schema.rs`: SQLite connection and table initialization
- `history.rs`: Transcription record CRUD
- `dictionary.rs`: Custom replacement terms
- `settings.rs`: Key-value settings store
- `stats.rs`: Usage statistics aggregation

### Models (src/models/)
- `manager.rs`: Model inventory, status tracking, path resolution
- `downloader.rs`: HTTP download with progress callbacks, resumable partial downloads

### Commands (src/commands/)
- One file per domain, each containing #[tauri::command] functions
- Bridges frontend requests to backend services

## State Management

### Backend (Rust)
Single `AppState` struct managed by Tauri, containing Mutex-wrapped subsystems:
- AudioCapture, WhisperLocal, TextInjector
- Database, ModelManager
- RecordingState, HotkeyMode

### Frontend (TypeScript)
Zustand stores with clear boundaries:
- `recordingStore`: Recording state machine, audio levels
- `historyStore`: Transcription records, search
- `settingsStore`: Persisted preferences
- `themeStore`: Light/dark/system appearance

## Security Model

- No sandbox (required for System Events automation)
- Audio processed in-memory only, never written to disk
- API keys stored in local SQLite (production: migrate to macOS Keychain)
- CSP restricts script/style sources
- No telemetry, no external analytics, no user accounts

## Performance Considerations

- Audio capture starts in <10ms (pre-allocated cpal stream)
- Whisper Base model transcribes 5s audio in ~200ms on M1
- Clipboard injection completes in ~50ms
- SQLite operations are negligible (<1ms)
- Model loading is one-time cost (~500ms for Base)
