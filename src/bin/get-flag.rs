use std::env;
use std::path::Path;
use std::process::exit;

use rusqlite::{params, Connection, Result};

const HELP: &str = "
Command to get a single flag

USAGE:
    get-flag <name>

name     Name of the flag

Try:

cargo run --bin get-flag name
";

#[derive(Debug)]
struct Flag {
    id: i32,
    name: String,
    value: bool,
}

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("{}", HELP);
        exit(1);
    }

    let name = args.remove(1);

    let path = Path::new("instance").join("flag.db");

    let conn = Connection::open(path)?;
    let result = conn.query_row(
        "SELECT id, name, value FROM flags WHERE name = ?",
        params![name],
        |row| {
            let value = match row.get(2)? {
                1 => true,
                _ => false,
            };

            Ok(Flag {
                id: row.get(0)?,
                name: row.get(1)?,
                value,
            })
        },
    );

    match result {
        Ok(flag) => {
            println!("Flag: {}: {}", flag.name, flag.value);
        }
        Err(_) => {
            println!("No Flag Found")
        }
    };

    Ok(())
}
