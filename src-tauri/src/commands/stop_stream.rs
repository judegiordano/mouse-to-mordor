use std::sync::Arc;

use tauri::State;

use crate::state::AppState;

pub async fn run(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    let mut guard = state.event_loop_channel.lock().map_err(|e| e.to_string())?;
    if let Some((tx, _label)) = guard.take() {
        let _ = tx.send(());
        Ok(())
    } else {
        Err("no active stream".into())
    }
}
