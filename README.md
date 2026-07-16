# Monochrome

> Minimalist, unlimited music streaming — desktop client for Windows.

Monochrome is a lightweight desktop wrapper for [monochrome.tf](https://monochrome.tf), built with Tauri v2. It adds native Windows integration on top of the web app: system tray, Discord Rich Presence, media key support, desktop notifications, and a custom download folder picker.

---

## Download

Grab the latest installer from [**Releases**](../../releases/latest).

| Format | Use case |
|--------|----------|
| `.msi` | Standard Windows installer (recommended) |
| `.exe` | NSIS standalone installer |

**Requirements:** Windows 10/11 with [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/) (pre-installed on Windows 11).

---

## Features

- Streams from [monochrome.tf](https://monochrome.tf) with full native window chrome
- **Discord Rich Presence** — track title, artist, album art, and playback timestamps
- **Media key support** — play/pause with your keyboard's media keys
- **System tray** — minimize to tray, show/hide, set download folder
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
| WebView2 | any | [microsoft.com](https://developer.microsoft.com/microsoft-edge/webview2/) |

### Running locally

```cmd
git clone https://github.com/Muhammad5777/monochrome-desktop.git
cd monochrome
"npm run dev.bat"
```

Or manually:

```cmd
npm install
npm run dev
```

The app will launch pointing at `https://monochrome.tf/library` by default. To develop against a local frontend, update `devUrl` in `src-tauri/tauri.conf.json`.

### Building a release

```cmd
npm install
npm run build
```

Outputs:
- `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/*.msi`
- `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/*.exe`

---

## Tech stack

- [Tauri v2](https://tauri.app) — Rust backend + WebView2 frontend
- [discord-rich-presence](https://crates.io/crates/discord-rich-presence) — Discord IPC
- [tauri-plugin-google-auth](https://crates.io/crates/tauri-plugin-google-auth) — Google OAuth
- [tauri-plugin-window-state](https://crates.io/crates/tauri-plugin-window-state) — persisted window size/position

---

## License

[MIT](LICENSE) © Samidy & Blacksigkill
