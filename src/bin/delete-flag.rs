use std::env;
use std::process::exit;

use rusqlite::Result;

use feature_flags::db::{delete_flag_by_name, get_db_rc};

const HELP: &str = "
Command to delete a single flag

USAGE:
    delete-flag <name>

name        Name of the flag

Try:

cargo run --bin delete-flag name
";

fn main() -> Result<()> {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        println!("{}", HELP);
        exit(1);
    }

    let name = args.remove(0);

    let conn = get_db_rc();
    let result = delete_flag_by_name(conn.clone(), name);
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
