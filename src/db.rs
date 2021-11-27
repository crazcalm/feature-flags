use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

pub type DBLite = Arc<Mutex<Connection>>;
pub type DBLocal = Rc<Connection>;

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

    Connection::open(path).expect("Unable to find the db")
}

pub fn get_db_rc() -> DBLocal {
    Rc::new(get_db())
}

pub fn get_db_server() -> DBLite {
    let conn = get_db();
    Arc::new(Mutex::new(conn))
}

pub fn initialize_db(conn: DBLocal) {
    conn.execute_batch(
        "DROP TABLE IF EXISTS flags;

        CREATE TABLE flags (
            id    INTEGER UNIQUE,
            name  TEXT NOT NULL UNIQUE,
            value INTEGER NOT NULL CHECK(value == 0 OR value == 1),
            PRIMARY KEY(id)
        );",
    )
    .expect("Error occured while trying to initialize the DB");

    println!("Successful Initialize the DB");
}

pub fn get_flag_by_name(conn: DBLocal, name: String) -> Result<FlagWithID, rusqlite::Error> {
    conn.query_row(
        "SELECT id, name, value FROM flags WHERE name = ?",
        params![name],
        |row| {
            let value = matches!(row.get(2)?, 1);

            Ok(FlagWithID {
                id: row.get(0)?,
                name: row.get(1)?,
                value,
            })
        },
    )
}

pub fn get_all_flags(conn: DBLocal) -> Result<Vec<FlagWithID>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, name, value FROM flags")?;

    let rows = stmt.query_map([], |row| {
        let value = matches!(row.get(2)?, 1);

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

pub fn delete_flag_by_name(conn: DBLocal, name: String) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM flags WHERE name = ?", params![name])
}

pub fn add_flag(conn: DBLocal, name: String, value: i32) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO flags (name, value) VALUES (?1, ?2)",
        params![name, value],
    )
}

pub fn update_flag(conn: DBLocal, name: String, value: i32) -> Result<usize, rusqlite::Error> {
    match get_flag_by_name(conn.clone(), name.clone()) {
        Ok(_) => {}
        Err(err) => {
            panic!("Error when updating the flag: {:?}", err);
        }
    }

    conn.execute(
        "UPDATE flags SET value = ? WHERE name = ?",
        params![value, name],
    )
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use rusqlite::Connection;

    use super::*;

    fn in_member_db() -> DBLocal {
        let conn = Connection::open_in_memory().unwrap();

        let local_conn = Rc::new(conn);

        // TODO: Update this call to return a Result
        initialize_db(local_conn.clone());

        local_conn
    }

    #[test]
    fn test_updating_a_flag() {
        let flag_name = "test_updating".to_string();

        let conn = in_member_db();

        // Initialize the flag to True
        let _ = add_flag(conn.clone(), flag_name.clone(), 1);

        let result = get_flag_by_name(conn.clone(), flag_name.clone()).unwrap();
        assert_eq!(result.value, true);

        // Update the flag value to False
        let _ = update_flag(conn.clone(), flag_name.clone(), 0).unwrap();

        let result = get_flag_by_name(conn.clone(), flag_name.clone()).unwrap();
        assert_eq!(result.value, false);
    }

    #[test]
    fn test_add_single_flag() {
        let flag_name = "test_flag".to_string();
        let flag_value_int = 1;
        let flag_value_bool = true;

        let conn = in_member_db();

        let _ = add_flag(conn.clone(), flag_name.clone(), flag_value_int.clone()).unwrap();

        let result = get_flag_by_name(conn.clone(), flag_name.clone()).unwrap();

        assert_eq!(result.name, flag_name);
        assert_eq!(result.value, flag_value_bool);
    }

    #[test]
    fn test_delete_flag() {
        let flag_name = "delete_test".to_string();

        let conn = in_member_db();

        let _ = add_flag(conn.clone(), flag_name.clone(), 1).unwrap();

        // Make sure the flag was added to the DB
        let flags = get_all_flags(conn.clone()).unwrap();
        assert_eq!(1, flags.len());

        // Delete flag
        let _ = delete_flag_by_name(conn.clone(), flag_name.clone()).unwrap();

        let flags = get_all_flags(conn.clone()).unwrap();
        assert_eq!(0, flags.len());
    }

    #[test]
    fn test_get_all_flags() {
        let conn = in_member_db();

        // Case: Zero Flags
        let result = get_all_flags(conn.clone()).unwrap();
        assert_eq!(0, result.len());

        // Case: More than Zero flags
        let flags = vec![
            ("test_1".to_string(), 0),
            ("test_2".to_string(), 1),
            ("test_3".to_string(), 0),
        ];
        let expected_num_of_flags = flags.len();

        for (name, value) in flags {
            let _ = add_flag(conn.clone(), name, value).unwrap();
        }

        let result = get_all_flags(conn.clone()).unwrap();
        assert_eq!(expected_num_of_flags, result.len());
    }
}
