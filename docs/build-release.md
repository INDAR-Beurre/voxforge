# Build & Release Guide

## Development Build

```bash
npm install
npm run tauri dev
```

## Production Build

```bash
npm run tauri build
```

Output: `src-tauri/target/release/bundle/macos/VoxForge.app`

## Universal Binary (Apple Silicon + Intel)

```bash
# Add Intel target
rustup target add x86_64-apple-darwin

# Build universal
npm run tauri build -- --target universal-apple-darwin
```

## Code Signing

### Prerequisites
- Apple Developer account
- Developer ID Application certificate in Keychain
- Notarization credentials

### Sign the app

```bash
export APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID)"

npm run tauri build
```

Tauri will automatically sign with the identity if `APPLE_SIGNING_IDENTITY` is set.

### Notarization

```bash
# Set credentials
export APPLE_ID="your@email.com"
export APPLE_PASSWORD="app-specific-password"
export APPLE_TEAM_ID="YOUR_TEAM_ID"

# Build and notarize
npm run tauri build
```

Or manually:

```bash
xcrun notarytool submit target/release/bundle/dmg/VoxForge_0.1.0_aarch64.dmg \
  --apple-id "$APPLE_ID" \
  --password "$APPLE_PASSWORD" \
  --team-id "$APPLE_TEAM_ID" \
  --wait

xcrun stapler staple target/release/bundle/dmg/VoxForge_0.1.0_aarch64.dmg
```

## Entitlements

The app requires these entitlements (configured in `src-tauri/Entitlements.plist`):
- `com.apple.security.device.audio-input` — microphone access
- `com.apple.security.automation.apple-events` — System Events control

App Sandbox is disabled because:
- Accessibility API requires it
- System Events automation requires it
- cpal audio capture works better without it

## DMG Customization

Edit `src-tauri/tauri.conf.json` under `bundle.macOS` to customize:
- Background image
- Icon positions
- Window size

## Minimum System Version

Set in `tauri.conf.json`:
```json
"macOS": {
  "minimumSystemVersion": "11.0"
}
```

This ensures compatibility with macOS Big Sur and later.

## Checklist

- [ ] Version bumped in `package.json` and `Cargo.toml`
- [ ] All features tested on Apple Silicon
- [ ] All features tested on Intel (if targeting universal binary)
- [ ] Code signing identity valid and not expired
- [ ] Notarization succeeds
- [ ] DMG opens cleanly and drag-to-Applications works
- [ ] First-launch permissions flow tested on clean system
- [ ] Microphone permission prompt appears
- [ ] Accessibility permission guidance works
- [ ] Model download completes successfully
- [ ] Global hotkey registers after accessibility grant
- [ ] Text injection works in: VS Code, Terminal, Safari, Slack, Notes
