#!/bin/bash
# Inject required Info.plist keys after Tauri bundling
APP_PATH="src-tauri/target/release/bundle/macos/VoxForge.app/Contents/Info.plist"

if [ -f "$APP_PATH" ]; then
  /usr/libexec/PlistBuddy -c "Delete :NSMicrophoneUsageDescription" "$APP_PATH" 2>/dev/null
  /usr/libexec/PlistBuddy -c "Add :NSMicrophoneUsageDescription string 'VoxForge needs microphone access to capture your voice for transcription.'" "$APP_PATH"

  /usr/libexec/PlistBuddy -c "Delete :NSAppleEventsUsageDescription" "$APP_PATH" 2>/dev/null
  /usr/libexec/PlistBuddy -c "Add :NSAppleEventsUsageDescription string 'VoxForge needs automation access to paste transcribed text into your apps.'" "$APP_PATH"

  echo "Info.plist keys injected successfully"
fi
