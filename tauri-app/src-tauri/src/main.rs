#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, WindowEvent,
};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_updater::UpdaterExt;

#[cfg(windows)]
use winapi::um::winbase::SetThreadExecutionState;

#[cfg(windows)]
const ES_CONTINUOUS: u32 = 0x80000000;
#[cfg(windows)]
const ES_SYSTEM_REQUIRED: u32 = 0x00000001;

#[derive(Deserialize)]
struct NotifyPayload {
    title: Option<String>,
    body: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    silent: bool,
}

#[tauri::command]
fn win_minimize(window: tauri::Window) {
    let _ = window.hide();
}

#[tauri::command]
fn win_close(window: tauri::Window) {
    let _ = window.hide();
}

#[tauri::command]
fn win_quit(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn win_pin(window: tauri::Window, pinned: bool) {
    let _ = window.set_always_on_top(pinned);
}

#[tauri::command]
fn win_notify(app: AppHandle, payload: NotifyPayload) {
    let title = payload.title.unwrap_or_else(|| "Norton Eventos".into());
    let body = payload.body.unwrap_or_default();
    let _ = app.notification().builder().title(title).body(body).show();
}

fn build_tray(app: &AppHandle) -> tauri::Result<()> {
    let open_i = MenuItem::with_id(app, "open", "Abrir Norton Eventos", true, None::<&str>)?;
    let pin_i = CheckMenuItem::with_id(app, "pin", "Manter no topo", true, true, None::<&str>)?;
    let sep = PredefinedMenuItem::separator(app)?;
    let quit_i = MenuItem::with_id(app, "quit", "Sair", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open_i, &pin_i, &sep, &quit_i])?;

    let pin_handle = pin_i.clone();

    let _tray = TrayIconBuilder::with_id("main")
        .tooltip("Norton Eventos \u{2014} MU Online")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "open" => {
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.show();
                    let _ = w.set_focus();
                }
            }
            "pin" => {
                if let Some(w) = app.get_webview_window("main") {
                    let checked = pin_handle.is_checked().unwrap_or(true);
                    let _ = w.set_always_on_top(checked);
                    let _ = app.emit("pin-changed", checked);
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(w) = app.get_webview_window("main") {
                    let visible = w.is_visible().unwrap_or(false);
                    let focused = w.is_focused().unwrap_or(false);
                    if visible && focused {
                        let _ = w.hide();
                    } else {
                        let _ = w.show();
                        let _ = w.set_focus();
                    }
                }
            } else if let TrayIconEvent::DoubleClick { .. } = event {
                let app = tray.app_handle();
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.show();
                    let _ = w.set_focus();
                }
            }
        })
        .build(app)?;
    Ok(())
}

async fn check_for_updates(handle: AppHandle) {
    let updater = match handle.updater() {
        Ok(u) => u,
        Err(_) => return,
    };
    let update = match updater.check().await {
        Ok(Some(u)) => u,
        _ => return,
    };
    let _ = update
        .download_and_install(|_chunk, _total| {}, || {})
        .await;
}

fn main() {
    #[cfg(windows)]
    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            win_minimize,
            win_close,
            win_quit,
            win_pin,
            win_notify
        ])
        .setup(|app| {
            build_tray(app.handle())?;

            let window = app.get_webview_window("main").unwrap();
            let win_clone = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = win_clone.hide();
                }
            });

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                check_for_updates(handle).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
