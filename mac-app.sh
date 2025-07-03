#!/bin/bash

PROJECT_DIR="$(pwd)"

# CONFIGURATION
APP_NAME="Offline Chess Puzzles"
EXECUTABLE_PATH="$PROJECT_DIR/target/release/offline-chess-puzzles"
ICON_IMAGE="$PROJECT_DIR/icon.png"
ICONSET_DIR="$PROJECT_DIR/target/offline-chess-puzzles.iconset"
ICNS_FILE_NAME="offline-chess-puzzles.icns"
ICNS_PATH="$PROJECT_DIR/target/$ICNS_FILE_NAME"
export APP_BUNDLE="$HOME/Applications/$APP_NAME.app"
MACOS_DIR="$APP_BUNDLE/Contents/MacOS"
RESOURCES_DIR="$APP_BUNDLE/Contents/Resources"
PLIST_PATH="$APP_BUNDLE/Contents/Info.plist"

# 0. Generate .icns from PNG
mkdir -p "$ICONSET_DIR"
sips -z 16 16     "$ICON_IMAGE" --out "$ICONSET_DIR/icon_16x16.png"
sips -z 32 32     "$ICON_IMAGE" --out "$ICONSET_DIR/icon_16x16@2x.png"
sips -z 32 32     "$ICON_IMAGE" --out "$ICONSET_DIR/icon_32x32.png"
sips -z 64 64     "$ICON_IMAGE" --out "$ICONSET_DIR/icon_32x32@2x.png"
sips -z 128 128   "$ICON_IMAGE" --out "$ICONSET_DIR/icon_128x128.png"
sips -z 256 256   "$ICON_IMAGE" --out "$ICONSET_DIR/icon_128x128@2x.png"
sips -z 256 256   "$ICON_IMAGE" --out "$ICONSET_DIR/icon_256x256.png"
sips -z 512 512   "$ICON_IMAGE" --out "$ICONSET_DIR/icon_256x256@2x.png"
sips -z 512 512   "$ICON_IMAGE" --out "$ICONSET_DIR/icon_512x512.png"
cp "$ICON_IMAGE" "$ICONSET_DIR/icon_512x512@2x.png"
iconutil -c icns "$ICONSET_DIR" -o "$ICNS_PATH"

# 1. Create app bundle structure
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

# 2. Copy the icon
cp "$ICNS_PATH" "$RESOURCES_DIR"

# 3. Create launcher script inside the app bundle
cat > "$MACOS_DIR/$APP_NAME" <<EOF
#!/bin/bash
cd "$PROJECT_DIR"
exec "$EXECUTABLE_PATH"
EOF

chmod +x "$MACOS_DIR/$APP_NAME"

# 4. Create Info.plist
cat > "$PLIST_PATH" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
 "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>$APP_NAME</string>
  <key>CFBundleIdentifier</key>
  <string>brianch.offlinechesspuzzles</string>
  <key>CFBundleName</key>
  <string>Offline Chess Puzzles</string>
  <key>CFBundleVersion</key>
  <string>1.0</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleIconFile</key>
  <string>$ICNS_FILE_NAME</string>
</dict>
</plist>
EOF

# 5. Refresh the app bundle so Spotlight recognizes it
touch "$APP_BUNDLE"

echo "✅ App bundle created at: $APP_BUNDLE"
echo "🎨 Icon added from: $ICON_IMAGE"
echo "📦 You can now find and launch it via Spotlight."
