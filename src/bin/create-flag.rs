use std::env;
use std::path::Path;
use std::process::exit;

use rusqlite::{params, Connection, Result};

const HELP: &str = "
Command to create a new flag

USAGE:
    create-flag <name> <value>

name     Name of the flag
value    Value of the flag (true or false)

Try:

cargo run --bin create-flag name value

";

fn valuate_value(arg: String) -> i32 {
    //TODO: Change this to a Result so that
    //      I can test this.

    match arg.to_lowercase().trim() {
        "true" => 1,
        "false" => 0,
        _ => {
            println!("Value much be 'true' or 'false'");
            exit(1);
        }
    }
}

fn main() -> Result<()> {
    // TODO: Move the arg parsing code into a function
    let mut args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("{}", HELP);
        exit(1);
    }

    let name = args.remove(1);
    let value = valuate_value(args.remove(1));

    // TODO: Figure out how to test this properly
    let path = Path::new("instance").join("flag.db");

    let conn = Connection::open(path)?;

    conn.execute(
        "INSERT INTO flags (name, value) VALUES (?1, ?2)",
        params![name, value],
    )?;

    println!("Done");

    Ok(())
}
