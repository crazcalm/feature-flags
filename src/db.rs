use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

pub type DBLite = Arc<Mutex<Connection>>;

#[derive(Debug, Serialize)]
pub struct FlagWithID {
    pub id: i32,
    pub name: String,
    pub value: bool,
}

#[derive(Debug, Deserialize)]
pub struct Flag {
    pub name: String,
    pub value: bool,
}

#[derive(Debug, Deserialize)]
pub struct FlagValue {
    pub value: bool,
}

pub fn get_db() -> Connection {
    let path = Path::new("instance").join("flag.db");

    let conn = Connection::open(path).expect("Unable to find the db");

    conn
}

pub fn get_db_rc() -> Rc<Connection> {
    Rc::new(get_db())
}

pub fn get_db_server() -> DBLite {
    let conn = get_db();
    Arc::new(Mutex::new(conn))
}

pub fn get_flag_by_name(conn: Rc<Connection>, name: String) -> Result<FlagWithID, rusqlite::Error> {
    let result = conn.query_row(
        "SELECT id, name, value FROM flags WHERE name = ?",
        params![name],
        |row| {
            let value = match row.get(2)? {
                1 => true,
                _ => false,
            };

            Ok(FlagWithID {
                id: row.get(0)?,
                name: row.get(1)?,
                value,
            })
        },
    );

    result
}

pub fn get_all_flags(conn: Rc<Connection>) -> Result<Vec<FlagWithID>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, name, value FROM flags")?;

    let rows = stmt.query_map([], |row| {
        let value = match row.get(2)? {
            1 => true,
            _ => false,
        };

        Ok(FlagWithID {
            id: row.get(0)?,
            name: row.get(1)?,
            value,
        })
    })?;

    // Convert rows to vec of items
    let mut result = vec![];
    for item in rows {
        result.push(item.unwrap())
    }

    Ok(result)
}

pub fn delete_flag_by_name(conn: Rc<Connection>, name: String) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM flags WHERE name = ?", params![name])
}

pub fn add_flag(conn: Rc<Connection>, name: String, value: i32) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO flags (name, value) VALUES (?1, ?2)",
        params![name, value],
    )
}
