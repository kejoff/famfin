/// API endpoint handlers
pub mod auth;
pub mod dashboard;
pub mod import;
pub mod transactions;
pub mod categories;
pub mod goals;
pub mod ml;

use std::sync::Arc;
use rusqlite::Connection;
use tokio::sync::Mutex;
use crate::ml::onnx::OnnxModel;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub ml_model: Arc<OnnxModel>,
}
