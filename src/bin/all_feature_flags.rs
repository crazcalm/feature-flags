use std::path::Path;

use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Flag {
    id: i32,
    name: String,
    value: bool,
}

fn main() -> Result<()> {
    let path = Path::new("instance").join("flag.db");

    let conn = Connection::open(path)?;

    let mut stmt = conn.prepare("SELECT id, name, value FROM flags")?;

    let flag_iter = stmt.query_map([], |row| {
        let value = match row.get(0)? {
            1 => true,
            _ => false,
        };

        Ok(Flag {
            id: row.get(0)?,
            name: row.get(1)?,
            value,
        })
    })?;

    for flag_result in flag_iter {
        let flag = flag_result.unwrap();
        println!("flag: {}: {}", flag.name, flag.value);
    }

    println!("Done");

    Ok(())
}
