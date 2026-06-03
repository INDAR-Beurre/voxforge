# VoxForge

Privacy-first voice dictation for macOS. Speak naturally, and text appears in your focused app.

VoxForge transcribes speech locally using Whisper and injects the result into any app — code editors, terminals, browsers, chat apps, notes — via clipboard-paste simulation. No audio ever leaves your machine in local mode.

## Features

- **On-device transcription** via whisper.cpp (tiny through large-v3 models)
- **Universal text injection** into any focused macOS app
- **Global hotkeys** — Push-to-Talk or Toggle modes
- **Floating widget** with recording state and audio visualization
- **Custom dictionary** for technical terms (camelCase, framework names, abbreviations)
- **Post-processing pipeline** — capitalization, filler word removal, punctuation
- **Local history** with search, favorites, and export
- **Usage analytics** — words, duration, WPM, daily activity
- **App-specific profiles** for different injection behaviors per app
- **Optional cloud transcription** via any OpenAI-compatible endpoint
- **Privacy center** explaining what stays local
- **Offline-first** — works without internet in local mode

## Requirements

- macOS 11+ (Big Sur or later)
- Apple Silicon (GPU-accelerated) or Intel (CPU fallback)
- Microphone permission
- Accessibility permission (for paste simulation and global shortcuts)

## Development Setup

### Prerequisites

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js (v18+)
brew install node

# System dependencies for whisper.cpp
xcode-select --install
```

### Install & Run

```bash
cd voxforge
npm install
npm run tauri dev
```

The app will open with hot-reload enabled for the frontend.

### Build for Release

```bash
npm run tauri build
```

The `.dmg` and `.app` bundle will be in `src-tauri/target/release/bundle/`.

## Architecture

```
voxforge/
├── src/                    # React + TypeScript frontend
│   ├── components/         # Reusable UI components
│   ├── pages/              # Route-level views
│   ├── stores/             # Zustand state management
│   ├── hooks/              # Custom React hooks
│   └── styles/             # Tailwind + global CSS
├── src-tauri/              # Rust backend
│   └── src/
│       ├── audio/          # Microphone capture, processing
│       ├── transcription/  # Whisper local + cloud abstraction
│       ├── injection/      # Text injection (clipboard/paste)
│       ├── database/       # SQLite persistence
│       ├── models/         # Model download & management
│       ├── commands/       # Tauri IPC command handlers
│       ├── state.rs        # Application state
│       ├── postprocess.rs  # Text cleanup rules
│       └── lib.rs          # Entry point
└── docs/                   # Documentation
```

## Permissions

| Permission | Purpose |
|-----------|---------|
| Microphone | Capture voice for transcription |
| Accessibility | Simulate Cmd+V paste and register global shortcuts |
| Network (optional) | Download models, cloud transcription |

## Model Sizes

| Model | Size | Speed | Accuracy | Recommended |
|-------|------|-------|----------|-------------|
| Tiny | 75 MB | Fastest | Basic | Testing only |
| Base | 142 MB | Fast | Good | Default |
| Small | 466 MB | Moderate | Better | Power users |
| Medium | 1.5 GB | Slow | High | Accuracy-first |
| Large v3 | 3.1 GB | Slowest | Best | Maximum quality |

## Configuration

Settings are stored in a local SQLite database at:
```
~/Library/Application Support/com.voxforge.app/voxforge.db
```

Models are stored at:
```
~/Library/Application Support/com.voxforge.app/models/
```

## License

MIT
