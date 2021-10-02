use std::sync::Arc;
use tokio::sync::Mutex;

use rusqlite::Connection;

pub type DBLite = Arc<Mutex<Connection>>;
