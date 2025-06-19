use rusqlite::Connection;
use std::sync::Mutex;

pub struct EmailDatabaseManager {
    db: Mutex<Connection>,
}

// pub struct
