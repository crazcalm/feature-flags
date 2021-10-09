use std::env;
use std::process::exit;

use feature_flags::db::{get_db_rc, get_flag_by_name};
use rusqlite::Result;

const HELP: &str = "
Command to get a single flag

USAGE:
    get-flag <name>

name     Name of the flag

Try:

cargo run --bin get-flag name
";

fn main() -> Result<()> {
    // Skipping the first arg because that is the program name
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        println!("{}", HELP);
        exit(1);
    }

    // get the flag name that was passed in
    let name = args.remove(0);

    // get db connection
    let conn = get_db_rc();

    let result = get_flag_by_name(conn.clone(), name);
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
