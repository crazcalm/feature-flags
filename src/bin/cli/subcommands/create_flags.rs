use feature_flags::db::{add_flag, DBLocal};

pub fn create_flag(db: DBLocal, name: String, value: i32) {
    let result = add_flag(db, name, value);

    // TODO: Make error message look better
    match result {
        Ok(_) => println!("Successfully added to the db"),
        Err(err) => println!("Failed to add to db: {:?}", err),
    }
}
