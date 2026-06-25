//! Lanjing Tauri 库入口。
//!
//! 负责注册插件、命令处理器，并启动应用。

use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use tauri::Manager;

/// `ping` 命令的响应体。
#[derive(Serialize)]
struct PingResponse {
    status: &'static str,
    timestamp: u64,
}

/// 向给定名称返回问候语。
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! Welcome to Lanjing.")
}

/// 返回应用存活状态与当前 Unix 毫秒时间戳。
#[tauri::command]
fn ping() -> PingResponse {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| u64::try_from(d.as_millis()).unwrap_or(0));
    PingResponse {
        status: "ok",
        timestamp,
    }
}

/// 构建并运行 Tauri 应用。
///
/// # Panics
///
/// 当 Tauri 应用启动失败时立即 panic，因为继续运行没有意义。
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        .invoke_handler(tauri::generate_handler![greet, ping])
        .run(tauri::generate_context!())
        .expect("error while running lanjing application");
}
