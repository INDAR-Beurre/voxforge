# Permission Flow

## Required Permissions

### 1. Microphone Access

**When requested:** On first recording attempt or during onboarding.

**macOS prompt:** "VoxForge would like to access the microphone."

**Why needed:** Core functionality — capturing voice audio for transcription.

**If denied:** App cannot function. Show a clear message directing user to System Settings > Privacy & Security > Microphone.

**Technical:** cpal requests permission automatically when opening an input stream. The system dialog appears once; subsequent launches use the cached decision.

### 2. Accessibility Access

**When requested:** During onboarding, before first text injection.

**macOS prompt:** User must manually grant in System Settings > Privacy & Security > Accessibility.

**Why needed:**
- Register global keyboard shortcuts that work regardless of focused app
- Simulate Cmd+V keypress via System Events for text injection

**If denied:** Global hotkeys won't work and text injection will fail. The app can still record and transcribe, but cannot inject text automatically.

**Technical:** AppleScript `tell application "System Events"` requires accessibility permission. The app should detect if permission is missing and guide the user.

### 3. Automation (System Events)

**When requested:** On first text injection attempt.

**macOS prompt:** "VoxForge would like to control System Events."

**Why needed:** Simulating keyboard shortcuts (Cmd+V) to paste transcribed text.

**If denied:** Text injection won't work. Clipboard will still contain the transcription but user must paste manually.

## Optional Permissions

### Network Access

**When needed:** Only when downloading Whisper models or using cloud transcription.

**If unavailable:** Local transcription works fully offline with pre-downloaded models.

## Onboarding Flow

```
1. Welcome screen
   ↓
2. "VoxForge needs microphone access to hear you."
   → Request microphone permission
   ↓
3. "VoxForge needs Accessibility to type into your apps."
   → Open System Settings link
   → Poll/check for permission grant
   ↓
4. "Choose your mode:"
   → Local only (offline, private)
   → Hybrid (local + optional cloud)
   ↓
5. "Download a transcription model"
   → Recommend Base (142 MB)
   → Show download progress
   ↓
6. "You're ready! Press Cmd+Shift+Space to dictate."
   → Show first-use tutorial overlay
```

## Permission Verification

The app should check permission status on each launch:
- Microphone: attempt to enumerate devices
- Accessibility: use AXIsProcessTrusted() or test System Events
- Display a non-blocking banner if any permission is missing

## Revoking Permissions

Users can revoke permissions at any time via System Settings. The app should handle revocation gracefully:
- Show a clear banner explaining what stopped working
- Provide a one-click path to the relevant System Settings pane
- Never crash or hang due to a missing permission
