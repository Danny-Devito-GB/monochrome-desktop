use crate::{get_source_url, open_external, set_source_url};
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::image::Image;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder};
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use tauri_plugin_notification::NotificationExt;

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

struct DiscordState {
    client: Mutex<DiscordIpcClient>,
    last_song: Mutex<Option<String>>,
}

struct DownloadState {
    path: Mutex<Option<PathBuf>>,
}

// ---------------------------------------------------------------------------
// Payload / parameter structs
// ---------------------------------------------------------------------------
/// Deserialized from the JS invoke call.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PresencePayload {
    title: String,
    artist: String,
    year: String,
    album: String,
    image: String,
    is_paused: bool,
    is_local: bool,
    start_timestamp: Option<i64>,
    end_timestamp: Option<i64>,
    track_url: String,
    artist_url: String,
    album_url: String,
    base_url: String,
}
/// Internal parameter bundle passed to `build_activity`.
struct ActivityParams<'a> {
    title: &'a str,
    state_text: &'a str,
    image: &'a str,
    album: &'a str,
    is_local: bool,
    is_paused: bool,
    start_timestamp: Option<i64>,
    end_timestamp: Option<i64>,
    listen_url: &'a str,
    track_url: &'a str,
    artist_url: &'a str,
    album_url: &'a str,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn save_download_path(app: &AppHandle, path: &Path) {
    if let Ok(config_dir) = app.path().app_config_dir() {
        if !config_dir.exists() {
            let _ = fs::create_dir_all(&config_dir);
        }
        let config_file = config_dir.join("download_path.txt");
        let _ = fs::write(config_file, path.to_string_lossy().as_bytes());
    }
}

fn load_download_path(app: &AppHandle) -> Option<PathBuf> {
    if let Ok(config_dir) = app.path().app_config_dir() {
        let config_file = config_dir.join("download_path.txt");
        if config_file.exists() {
            if let Ok(content) = fs::read_to_string(config_file) {
                return Some(PathBuf::from(content.trim()));
            }
        }
    }
    None
}

/// Builds a typed Discord `Activity`.
fn build_activity(p: ActivityParams<'_>) -> activity::Activity<'_> {
    let local_note =
        "The current track is a local file \u{2014} links and artwork may be unavailable";

    // ── Assets ──────────────────────────────────────────────────────────────
    let mut assets = activity::Assets::new()
        .large_image(p.image)
        .large_text(p.album);

    if p.is_local {
        assets = assets.small_image("logo").small_text(local_note);
    }

    if !p.album_url.is_empty() {
        assets = assets.large_url(p.album_url);
    }

    // ── Core activity ────────────────────────────────────────────────────────
    let mut act = activity::Activity::new()
        .activity_type(activity::ActivityType::Listening)
        .details(p.title)
        .state(p.state_text)
        .assets(assets)
        .buttons(vec![activity::Button::new(
            "Listen On Monochrome",
            p.listen_url,
        )]);

    // Clickable URLs (only if valid URLs are provided)
    if !p.track_url.is_empty() {
        act = act.details_url(p.track_url);
    }
    if !p.artist_url.is_empty() {
        act = act.state_url(p.artist_url);
    }

    // ── Timestamps (only when playing; JS sends null when paused) ────────────
    if !p.is_paused {
        if let Some(start) = p.start_timestamp {
            let mut ts = activity::Timestamps::new().start(start);
            if let Some(end) = p.end_timestamp {
                ts = ts.end(end);
            }
            act = act.timestamps(ts);
        }
    }

    act
}

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------

#[tauri::command]
fn update_discord_presence(
    app: AppHandle,
    state: tauri::State<DiscordState>,
    payload: PresencePayload,
) -> Result<(), String> {
    let PresencePayload {
        title,
        artist,
        year,
        album,
        image,
        is_paused,
        is_local,
        start_timestamp,
        end_timestamp,
        track_url,
        artist_url,
        album_url,
        base_url,
    } = payload;

    // Pad short strings to satisfy Discord's 2-character minimum
    let title = if title.len() < 2 {
        format!("{}  ", title)
    } else {
        title
    };

    // Build state string: "Artist • Year" or just "Artist"
    let mut state_text = if !year.is_empty() {
        format!("{} \u{2022} {}", artist, year)
    } else {
        artist.clone()
    };

    if state_text.len() < 2 {
        state_text = format!("{}  ", state_text);
    }

    if is_paused {
        state_text = format!("{} (Paused)", state_text);
    }

    let listen_url: &str = if !track_url.is_empty() {
        &track_url
    } else {
        &base_url
    };

    let act = build_activity(ActivityParams {
        title: &title,
        state_text: &state_text,
        image: &image,
        album: &album,
        is_local,
        is_paused,
        start_timestamp,
        end_timestamp,
        listen_url,
        track_url: &track_url,
        artist_url: &artist_url,
        album_url: &album_url,
    });

    // Debug: print what's being sent to Discord
    // To activate this, uncomment the line below and ensure you have a console attached (e.g. by running `cargo tauri dev`).
    // println!("[Discord RPC] Sending activity: title={:?}, artist={:?}, album={:?}, image={:?}, paused={}, is_local={}",title, state_text, album, image, is_paused, is_local);

    let mut client_guard = state.client.lock().map_err(|_| "Failed to lock mutex")?;

    if let Err(e) = client_guard.set_activity(act.clone()) {
        let _ = client_guard.close();
        if client_guard.connect().is_ok() {
            client_guard
                .set_activity(act)
                .map_err(|e2| format!("Failed to reconnect to Discord: {} / {}", e, e2))?;
        } else {
            return Err(format!("Failed to connect to Discord: {}", e));
        }
    }

    // Notification on track change (only while playing)
    if !is_paused {
        let mut last_song_guard = state.last_song.lock().unwrap();
        let current_song_key = format!("{} - {}", title, artist);

        if last_song_guard.as_deref() != Some(&current_song_key) {
            *last_song_guard = Some(current_song_key);

            if let Some(win) = app.get_webview_window("main") {
                if !win.is_focused().unwrap_or(false) {
                    let _ = app
                        .notification()
                        .builder()
                        .title("Now Playing")
                        .body(format!("{}\n{}", title, artist))
                        .show();
                }
            }
        }
    }

    Ok(())
}

#[tauri::command]
fn clear_discord_presence(state: tauri::State<DiscordState>) -> Result<(), String> {
    let mut client_guard = state.client.lock().map_err(|_| "Failed to lock mutex")?;

    if let Err(e) = client_guard.clear_activity() {
        let _ = client_guard.close();
        if client_guard.connect().is_ok() {
            client_guard
                .clear_activity()
                .map_err(|e2| format!("Failed to reconnect to Discord: {} / {}", e, e2))?;
        } else {
            return Err(format!("Failed to connect to Discord: {}", e));
        }
    }

    // Clear last song tracking
    *state.last_song.lock().unwrap() = None;

    Ok(())
}

// ---------------------------------------------------------------------------
// Builder configuration (plugins, state, commands)
// ---------------------------------------------------------------------------

pub fn configure(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    let client_id = "1495102913356501082";

    // DiscordIpcClient::new is infallible in v1.x; connect separately.
    let mut client = DiscordIpcClient::new(client_id);
    let _ = client.connect();

    builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(
            |app: &AppHandle, _args, _cwd| {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            },
        ))
        .manage(DiscordState {
            client: Mutex::new(client),
            last_song: Mutex::new(None),
        })
        .manage(DownloadState {
            path: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            update_discord_presence,
            clear_discord_presence,
            open_external,
            get_source_url,
            set_source_url
        ])
}

// ---------------------------------------------------------------------------
// App setup (window, tray, shortcuts)
// ---------------------------------------------------------------------------

pub fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(config_dir) = app.path().app_config_dir() {
        if !config_dir.exists() {
            let _ = fs::create_dir_all(&config_dir);
        }
    }

    let state = app.state::<DownloadState>();
    *state.path.lock().unwrap() = load_download_path(app.handle());

    // System tray
    let quit = MenuItemBuilder::with_id("quit", "Quit Monochrome").build(app)?;
    let show = MenuItemBuilder::with_id("show", "Show Player").build(app)?;
    let change_dl = MenuItemBuilder::with_id("change_dl", "Set Download Folder").build(app)?;
    let menu = MenuBuilder::new(app)
        .item(&show)
        .item(&change_dl)
        .separator()
        .item(&quit)
        .build()?;

    let icon = Image::from_bytes(include_bytes!("../icons/icon.png")).expect("Failed to load icon");
    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "quit" => std::process::exit(0),
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "change_dl" => {
                let app_handle = app.clone();
                app.dialog().file().pick_folder(move |folder| {
                    if let Some(path) = folder {
                        let path = path.into_path().unwrap();
                        let state = app_handle.state::<DownloadState>();
                        *state.path.lock().unwrap() = Some(path.clone());
                        save_download_path(&app_handle, &path);
                    }
                });
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click {
                button,
                button_state,
                ..
            } = event
            {
                if button == MouseButton::Left && button_state == MouseButtonState::Up {
                    let app = tray.app_handle();
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    // Global media key shortcut
    let _ = app
        .global_shortcut()
        .on_shortcut("MediaPlayPause", |app, _shortcut, event| {
            if event.state == ShortcutState::Released {
                let _ = app.emit("media-toggle", ());
            }
        });

    // Main window
    let app_handle = app.handle().clone();
    let source_url = crate::load_source_url(app.handle());
    let mut init_script = String::new();
    // Commenting out the discord presence bridge script for now, as I've pushed it to the monochrome source
    // init_script.push_str(include_str!(
    //     "../scripts/desktop/discord_presence_bridge.js"
    // ));
    init_script.push_str("document.addEventListener('contextmenu', e => e.preventDefault());");
    init_script.push('\n');
    init_script.push_str(include_str!("../scripts/desktop/external_link_router.js"));
    init_script.push('\n');
    let fallback_script = include_str!("../scripts/desktop/source_url_fallback.js")
        .replace("__EXPECTED_URL__", &source_url)
        .replace("__DEFAULT_URL__", crate::DEFAULT_SOURCE_URL);
    init_script.push_str(&fallback_script);

    let window = WebviewWindowBuilder::new(
        app,
        "main",
        WebviewUrl::External(source_url.parse().unwrap()),
    )
    .title("Monochrome")
    .inner_size(1200.0, 800.0)
    .initialization_script(&init_script)
    .on_download(move |_webview, event| {
        if let tauri::webview::DownloadEvent::Requested { destination, .. } = event {
            let state = app_handle.state::<DownloadState>();
            let path_guard = state.path.lock().unwrap();
            if let Some(path) = &*path_guard {
                if let Some(name) = destination.file_name() {
                    *destination = path.join(name);
                }
            }
        }
        true
    })
    .build()?;

    let _ = window.show();
    let _ = window.set_theme(Some(tauri::Theme::Dark));

    let window_clone = window.clone();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            let _ = window_clone.hide();
            api.prevent_close();
        }
    });

    Ok(())
}
