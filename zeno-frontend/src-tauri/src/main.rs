#![windows_subsystem = "windows"]

use zeno_lib::commands::*;
use zeno_lib::AppState;
use tauri::{
    Manager,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    image::Image,
    webview::WebviewWindowBuilder,
    WebviewUrl,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

fn load_icon() -> Option<Image<'static>> {
    let icon_bytes = include_bytes!("../icons/icon.png");
    let img = image::load_from_memory(icon_bytes).ok()?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let data = rgba.into_raw();
    Some(Image::new_owned(data, width, height))
}

// 全局热键：Alt+Q
const QUICK_HOTKEY: &str = "alt+q";

// 将快速搜索窗口定位到光标所在显示器的左上角，然后显示并聚焦
fn show_quick_window(app: &tauri::AppHandle) {
    let Some(win) = app.get_webview_window("quick") else {
        return;
    };

    if let Ok(cursor) = app.cursor_position() {
        let monitors = app.available_monitors().unwrap_or_default();
        let monitor = monitors.into_iter().find(|m| {
            let pos = m.position();
            let size = m.size();
            cursor.x >= pos.x as f64
                && cursor.x < (pos.x + size.width as i32) as f64
                && cursor.y >= pos.y as f64
                && cursor.y < (pos.y + size.height as i32) as f64
        });
        if let Some(m) = monitor {
            let mpos = m.position();
            let _ = win.set_position(tauri::PhysicalPosition::new(mpos.x + 8, mpos.y + 8));
        } else {
            let _ = win.center();
        }
    } else {
        let _ = win.center();
    }

    let _ = win.show();
    let _ = win.set_focus();
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            check_vault_exists,
            setup_master_password,
            auto_unlock,
            get_records,
            add_record,
            update_record,
            delete_record,
            search_records,
            generate_password,
            hide_quick,
        ])
        .setup(|app| {
            let icon = load_icon().unwrap_or_else(|| {
                app.default_window_icon().unwrap().clone()
            });

            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_icon(icon.clone());
            }

            // 预创建快速搜索窗口（隐藏），热键唤起时零延迟
            let quick = WebviewWindowBuilder::new(
                app,
                "quick",
                WebviewUrl::App("quick.html".into()),
            )
            .title("Zeno 快速搜索")
            .inner_size(480.0, 430.0)
            .decorations(false)
            .resizable(false)
            .always_on_top(true)
            .skip_taskbar(true)
            .visible(false)
            .build()?;
            let _ = quick.set_icon(icon.clone());

            // 注册全局热键 Alt+Q，按下时切换快速搜索窗口
            app.global_shortcut().on_shortcut(QUICK_HOTKEY, |app, _shortcut, event| {
                if event.state == ShortcutState::Pressed {
                    let Some(win) = app.get_webview_window("quick") else {
                        return;
                    };
                    if win.is_visible().unwrap_or(false) {
                        let _ = win.hide();
                    } else {
                        show_quick_window(app);
                    }
                }
            })?;

            let show_item = MenuItemBuilder::with_id("show", "显示主窗口")
                .build(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出")
                .build(app)?;

            let menu = MenuBuilder::new(app)
                .item(&show_item)
                .item(&quit_item)
                .build()?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("Zeno - 凭据管理")
                .icon(icon)
                .on_menu_event(move |app, event| {
                    match event.id().as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::DoubleClick { .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    window.hide().unwrap();
                    api.prevent_close();
                }
                // 快速搜索窗口失焦时自动隐藏
                tauri::WindowEvent::Focused(false) => {
                    if window.label() == "quick" {
                        let _ = window.hide();
                    }
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
