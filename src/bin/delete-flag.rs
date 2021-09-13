use std::env;
use std::path::Path;
use std::process::exit;

use rusqlite::{params, Connection, Result};

const HELP: &str = "
Command to delete a single flag

USAGE:
    delete-flag <name>

name        Name of the flag

Try:

cargo run --bin delete-flag name
";

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("{}", HELP);
        exit(1);
    }

    let name = args.remove(1);

    let path = Path::new("instance").join("flag.db");

    let conn = Connection::open(path)?;
    let result = conn.execute("DELETE FROM flags WHERE name = ?", params![name]);

    match result {
        Ok(deleted) => {
            println!("{} rows where deleted", deleted);
        }
        Err(err) => {
            println!("delete failed: {}", err);
        }
    };

    Ok(())
}
