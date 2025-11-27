use serde_json::Value;
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};

use crate::app_config::AppConfig;

#[derive(Default, Debug)]
pub struct AppState {
    pub event_loop_channel: Arc<Mutex<Option<(oneshot::Sender<()>, String)>>>,
    // pub event_loop_channel:
    //     Arc<Mutex<Option<(oneshot::Sender<()>, String, mpsc::UnboundedSender<Value>)>>>,
    pub config: AppConfig,
}
