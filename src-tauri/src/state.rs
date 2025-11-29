use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

use crate::app_config::AppConfig;

#[derive(Default, Debug)]
pub struct AppState {
    pub event_loop_channel: Arc<Mutex<Option<(oneshot::Sender<()>, String)>>>,
    pub config: AppConfig,
}
