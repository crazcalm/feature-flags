use std::env;
use std::process::exit;

use feature_flags::db;

const HELP: &str = "
Command to create a new flag

USAGE:
    create-flag <name> <value>

name     Name of the flag
value    Value of the flag (true or false)

Try:

cargo run --bin create-flag name value

";

fn valuate_value(arg: String) -> Result<i32, String> {
    //TODO: Change this to a Result so that
    //      I can test this.

    match arg.to_lowercase().trim() {
        "true" => Ok(1),
        "false" => Ok(0),
        _ => Err("Value much be 'true' or 'false'".to_string()),
    }
}

fn main() -> Result<(), rusqlite::Error> {
    // TODO: Move the arg parsing code into a function
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        println!("{}", HELP);
        exit(1);
    }

    let name = args.remove(0);
    let value = match valuate_value(args.remove(0)) {
        Ok(value) => value,
        Err(err) => panic!("{}", err),
    };

    let conn = db::get_db_rc();

    let result = db::add_flag(conn.clone(), name, value);

    // TODO: Make error message look better
    match result {
        Ok(_) => println!("Successfully added to the db"),
        Err(err) => println!("Failed to add to db: {:?}", err),
    }

    Ok(())
}
