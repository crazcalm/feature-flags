use feature_flags::db::{get_db_rc, initialize_db};

fn main() {
    let conn = get_db_rc();

    initialize_db(conn).expect("Unable to initialize DB");
}
