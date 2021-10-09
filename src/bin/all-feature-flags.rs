use rusqlite::Result;

use feature_flags::db::{get_all_flags, get_db_rc};

fn main() -> Result<()> {
    let conn = get_db_rc();

    let rows = get_all_flags(conn.clone()).expect("Unable to get all flags");
    for flag in rows {
        println!("flag: {}: {}", flag.name, flag.value);
    }
    println!("Done");

    Ok(())
}
