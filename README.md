# Monochrome-Desktop

> Minimalist, unlimited music streaming — desktop client for Windows and Linux.

Monochrome-Desktop is a lightweight desktop wrapper for [monochrome.tf](https://github.com/monochrome-music/monochrome), built with Tauri v2. It adds native desktop integration on top of the web app: system tray, Discord Rich Presence, media key support, desktop notifications, launch-at-startup, and a custom download folder picker.

---

## Download

Grab the latest installer from [**Releases**](../../releases/latest).

| Format | Platform | Use case |
|--------|----------|----------|
| `.msi` | Windows | Standard Windows installer (recommended) |
| `.exe` | Windows | NSIS standalone installer |
| `.deb` | Linux (Debian) | Standard Debian package (recommended) |
| `.AppImage` | Linux (Universal) | Portable, no install required |

**Windows requirements:** Windows 10/11 with [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/) (pre-installed on Windows 11).

**Linux requirements:** A distro with WebKitGTK available (Debian 12+, Ubuntu 22.04+, Linux Mint 21+, and derivatives). The `.deb` package pulls in its runtime dependencies automatically via `apt`.

> **Discord Rich Presence on Linux:** Works with the official Discord client or Vesktop's `.deb` build (enable "Rich Presence via arRPC" in Vesktop's settings). Snap and Flatpak installs of either sandbox the IPC socket and won't connect.

---

## Features

- Streams from [monochrome.tf](https://monochrome.tf) with full native window chrome
- **Discord Rich Presence** — track title, artist, album art, and playback timestamps
- **Media key support** — play/pause with your keyboard's media keys
- **System tray** — minimize to tray, show/hide, set download folder
- **Launch at startup** — toggle from the tray menu (autostart entry on Windows and Linux)
- **Desktop notifications** — now playing alerts when the window is in the background
- **Single instance** — re-focuses the existing window if launched again
- **Custom source URL** — point the app at a local dev build or self-hosted instance via the fallback screen

---

## Development

### Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Node.js | 20+ | [nodejs.org](https://nodejs.org) |
| Rust | stable | [rustup.rs](https://rustup.rs) |
| WebView2 (Windows only) | any | [microsoft.com](https://developer.microsoft.com/microsoft-edge/webview2/) |

**Linux (Debian-based) system dependencies:**
```bash
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libgtk-3-dev \
  libxdo-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  patchelf \
  build-essential \
  curl wget file libssl-dev
```

### Running locally
```cmd
git clone https://github.com/Danny-Devito-GB/monochrome-desktop.git
cd monochrome-desktop
npm install
npm run dev
```

The app will launch pointing at `https://monochrome.tf/` by default. To develop against a local frontend, update `devUrl` in `src-tauri/tauri.conf.json`.

### Building a release

```cmd
npm install
npm run build
```

**Windows outputs:**
- `src-tauri/target/release/bundle/msi/*.msi`
- `src-tauri/target/release/bundle/nsis/*.exe`

**Linux outputs:**
- `src-tauri/target/release/bundle/deb/*.deb`
- `src-tauri/target/release/bundle/appimage/*.AppImage`

---

## License

[MIT](LICENSE) © Samidy & Blacksigkill
