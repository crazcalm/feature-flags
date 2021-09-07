use std::path::Path;

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_name = "flag.db";
    let path = Path::new("instance").join(db_name);

    let conn = Connection::open(path)?;

    conn.execute_batch(
        "DROP TABLE IF EXISTS flags;

        CREATE TABLE flags (
            id    INTEGER UNIQUE,
            name  TEXT NOT NULL UNIQUE,
            value INTEGER NOT NULL CHECK(value == 0 OR value == 1),
            PRIMARY KEY(id)
        );",
    )?;

    println!("Successful");

    Ok(())
}
