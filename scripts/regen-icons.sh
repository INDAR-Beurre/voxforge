#!/bin/bash
# Regenerate all icon files from scripts/icon-source.svg
# Source of truth: scripts/icon-source.svg
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC_SVG="$ROOT/scripts/icon-source.svg"
TMP_PNG="$(mktemp -t voxforge-icon-XXXXXX).png"
ICONSET_DIR="$(mktemp -d -t voxforge-iconset-XXXXXX)"
ICONSET="$ICONSET_DIR.iconset"
DST="$ROOT/src-tauri/icons"
mkdir -p "$ICONSET"

if [ ! -f "$SRC_SVG" ]; then
  echo "Source SVG not found: $SRC_SVG" >&2
  exit 1
fi

# Force 32-bit RGBA PNG output (Tauri requires true RGBA, not palette)
PNG_OUT() { magick "$@" "PNG32:$(mktemp -t voxforge-png-XXXXXX).png"; }

# Render master 1024x1024 PNG (RGBA, 8-bit)
magick -background none -density 600 "$SRC_SVG" -resize 1024x1024 -depth 8 -define png:color-type=6 "PNG32:$TMP_PNG"

# Tauri standard PNGs
magick "$TMP_PNG" -resize 32x32   -depth 8 -define png:color-type=6 "PNG32:$DST/32x32.png"
magick "$TMP_PNG" -resize 128x128 -depth 8 -define png:color-type=6 "PNG32:$DST/128x128.png"
magick "$TMP_PNG" -resize 256x256 -depth 8 -define png:color-type=6 "PNG32:$DST/128x128@2x.png"
magick "$TMP_PNG" -resize 512x512 -depth 8 -define png:color-type=6 "PNG32:$DST/icon.png"

# Windows store square logos
for size in 30 44 71 89 107 142 150 284 310; do
  magick "$TMP_PNG" -resize ${size}x${size} -depth 8 -define png:color-type=6 "PNG32:$DST/Square${size}x${size}Logo.png"
done
magick "$TMP_PNG" -resize 50x50 -depth 8 -define png:color-type=6 "PNG32:$DST/StoreLogo.png"

# .iconset for iconutil
magick "$TMP_PNG" -resize 16x16     -depth 8 -define png:color-type=6 "PNG32:$ICONSET/icon_16x16.png"
magick "$TMP_PNG" -resize 32x32     -depth 8 -define png:color-type=6 "PNG32:$ICONSET/icon_16x16@2x.png"
magick "$TMP_PNG" -resize 32x32     -depth 8 -define png:color-type=6 "PNG32:$ICONSET/icon_32x32.png"
magick "$TMP_PNG" -resize 64x64     -depth 8 -define png:color-type=6 "PNG32:$ICONSET/icon_32x32@2x.png"
magick "$TMP_PNG" -resize 128x128   -depth 8 -define png:color-type=6 "PNG32:$ICONSET/icon_128x128.png"
magick "$TMP_PNG" -resize 256x256   -depth 8 -define png:color-type=6 "PNG32:$ICONSET/icon_128x128@2x.png"
magick "$TMP_PNG" -resize 256x256   -depth 8 -define png:color-type=6 "PNG32:$ICONSET/icon_256x256.png"
magick "$TMP_PNG" -resize 512x512   -depth 8 -define png:color-type=6 "PNG32:$ICONSET/icon_256x256@2x.png"
magick "$TMP_PNG" -resize 512x512   -depth 8 -define png:color-type=6 "PNG32:$ICONSET/icon_512x512.png"
magick "$TMP_PNG" -resize 1024x1024 -depth 8 -define png:color-type=6 "PNG32:$ICONSET/icon_512x512@2x.png"

iconutil -c icns "$ICONSET" -o "$DST/icon.icns"
magick "$TMP_PNG" -resize 256x256 -depth 8 "$DST/icon.ico"

rm -f "$TMP_PNG"
rm -rf "$ICONSET" "$ICONSET_DIR"

echo "All icon files regenerated from $SRC_SVG (RGBA 8-bit)"
