use feature_flags::db;

pub fn update_flag(conn: db::DBLocal, name: String, value: i32) {
    let result = db::update_flag(conn.clone(), name, value);

    match result {
        Ok(_) => println!("Successfully updated the db"),
        Err(err) => println!("Failed to add to db: {:?}", err),
    }
}
