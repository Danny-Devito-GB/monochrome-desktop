#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    // On Linux, default to X11 so KDE draws the titlebar instead of GTK's
    // thick Wayland one. Must run before GTK starts; respects a user-set backend.    #[cfg(target_os = "linux")]
    if std::env::var_os("GDK_BACKEND").is_none() {
        std::env::set_var("GDK_BACKEND", "x11");
    }

    monochrome_lib::run();
}
