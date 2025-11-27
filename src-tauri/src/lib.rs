use anyhow::Result;
use mouse_position::mouse_position::Mouse;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;
use tauri::{Emitter, State};
use tokio::sync::{mpsc, oneshot};

use crate::app_config::{AppConfig, DistanceTraveled, APP_NAME, CONFIG_NAME};
use crate::state::AppState;

pub mod app_config;
mod commands;
pub mod rest_client;
mod rtc;
mod state;

// conversions
const INCHES_IN_PIXELS: f64 = 0.010417_f64;
const INCHES_IN_FEET: f64 = 12.0_f64;
const FEET_IN_MILES: f64 = 5280.0_f64;

#[tauri::command(rename_all = "snake_case")]
async fn start_stream(
    window: tauri::Window,
    state: State<'_, Arc<AppState>>,
) -> Result<(), String> {
    let mut guard = state.event_loop_channel.lock().map_err(|e| e.to_string())?;
    if guard.is_some() {
        if let Some((old_tx, _)) = guard.take() {
            let _ = old_tx.send(());
        }
    }
    let (tx, rx) = oneshot::channel::<()>();
    let win = window.clone();
    let config = state.config.clone();

    tokio::spawn(async move {
        let config = config.clone();
        let mut interval = tokio::time::interval(Duration::from_millis(16));
        let mut rx = rx;

        let mut last_position = (0i32, 0i32);
        let mut total_pixels_traveled = config.distance_traveled.total_pixels_traveled;
        let mut total_inches_traveled = config.distance_traveled.total_inches_traveled;
        let mut total_feet_traveled = config.distance_traveled.total_feet_traveled;
        let mut total_miles_traveled = config.distance_traveled.total_miles_traveled;

        let mut last_emit = tokio::time::Instant::now();
        let mut last_store = tokio::time::Instant::now();
        let emit_interval = Duration::from_millis(200);
        let store_interval = Duration::from_secs(5);

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    match Mouse::get_mouse_position() {
                        Mouse::Position { x, y } => {
                            let dx = (x - last_position.0) as f64;
                            let dy = (y - last_position.1) as f64;
                            let delta = (dx*dx + dy*dy).sqrt();

														if delta > 0.0 {
															total_pixels_traveled += delta;
															total_inches_traveled = (total_pixels_traveled * INCHES_IN_PIXELS).round();
															total_feet_traveled = (total_inches_traveled / INCHES_IN_FEET).round();
															total_miles_traveled = (total_feet_traveled / FEET_IN_MILES).round();

															let distance_traveled = DistanceTraveled {
																	total_pixels_traveled: total_pixels_traveled.round(),
																	total_inches_traveled,
																	total_feet_traveled,
																	total_miles_traveled,
															};

															if last_emit.elapsed() >= emit_interval {
																	let _ = window.emit("distance-traveled", &distance_traveled);
																	last_emit = tokio::time::Instant::now();
															}

															if last_store.elapsed() >= store_interval {
																	let updates = AppConfig {
																			distance_traveled: distance_traveled.clone(),
																			..config.clone()
																	};
																	tokio::task::spawn_blocking(move || {
																			if let Err(e) = confy::store(APP_NAME, CONFIG_NAME, updates) {
																					eprintln!("error storing config: {}", e);
																			}
																	});
																	last_store = tokio::time::Instant::now();
															}
                            }

                            last_position.0 = x;
                            last_position.1 = y;
                        }
                        Mouse::Error => {
                            eprintln!("error getting mouse position");
                        }
                    }
                }
                _ = &mut rx => {
                    break;
                }
            }
        }
    });
    *guard = Some((tx, win.label().to_string()));
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn stop_stream(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let mut guard = state.event_loop_channel.lock().map_err(|e| e.to_string())?;
    if let Some((tx, _label)) = guard.take() {
        let _ = tx.send(());
        Ok(())
    } else {
        Err("no active stream".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config = AppConfig::load();
    let state = AppState {
        config,
        ..Default::default()
    };

    tauri::Builder::default()
        .manage(Arc::new(state))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_stream, stop_stream])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
