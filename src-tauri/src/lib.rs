use anyhow::Result;
use std::sync::Arc;
use tauri::State;

use crate::app_config::AppConfig;
use crate::state::AppState;

pub mod app_config;
mod commands;
pub mod constants;
mod state;

#[tauri::command(rename_all = "snake_case")]
async fn start_stream(
    window: tauri::Window,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    commands::start_stream::run(window, state).await
}

#[tauri::command(rename_all = "snake_case")]
async fn stop_stream(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    commands::stop_stream::run(state).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState {
        config: AppConfig::load(),
        ..Default::default()
    };
    tauri::Builder::default()
        .manage(Arc::new(state))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_stream, stop_stream])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
