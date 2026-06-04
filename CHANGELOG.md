# Changelog

All notable changes to VoxForge are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] — 2026-06-04

First stable release. VoxForge is feature-complete for local voice dictation on macOS.

### Highlights
- Push-to-talk dictation with a floating waveform widget
- On-device Whisper transcription (tiny → large-v3)
- Auto language detection across English, French, German, and more
- One-click permission flows for Accessibility and Microphone
- Universal text injection into any focused macOS app

### Added
- Core dictation pipeline: microphone capture, resampling, Whisper transcription, clipboard-paste injection
- Floating widget with live audio level visualization (auto-hides when not recording)
- Global hotkey support with native `Control+Shift+S` binding (and optional fn/Globe key via CGEventTap)
- Custom dictionary for camelCase, abbreviations, and technical terms
- Local SQLite history with search, favorites, and re-inject
- Daily usage statistics (words, duration, WPM)
- App-specific profiles for per-app injection behavior
- Post-processing pipeline: capitalization, filler-word removal, punctuation normalization
- Optional OpenAI-compatible cloud transcription endpoint
- Localized language selection (auto, en, fr, de, es, ja, zh, ko, pt)
- Privacy center explaining what stays local
- Redesigned app icon (mic + forge-ember waves) regenerated from a single SVG source

### Fixed
- True window transparency for the floating widget (macOS private API)
- Robust audio capture stream lifecycle (no leaked `Stream` handles)
- Auto-loading of the first downloaded Whisper model on startup
- `Cmd+V` injection via `CGEventPost` (no more `osascript` failures)
- Permission prompts no longer re-fire on every launch
- 8-bit RGBA icon set to satisfy Tauri 2.11's strict icon validator

### Changed
- Hardened copy in error states ("No audio captured", "No speech detected", "No model loaded")
- Tighter design system across the app (Dia-Browser-inspired surface tokens)
- Settings reorganized to put Permissions at the top when access is missing

## [0.x] — pre-release

Iterative development builds. Earlier tags (v0.1.x, v0.2.x) are kept in the
GitHub releases page for archival purposes.
