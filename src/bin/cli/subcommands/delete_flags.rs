use feature_flags::db::{delete_flag_by_name, DBLocal};

pub fn delete_flag(db: DBLocal, name: String) {
    let result = delete_flag_by_name(db.clone(), name);
    match result {
        Ok(deleted) => {
            println!("{} row deleted", deleted);
        }
        Err(err) => {
            println!("delete failed: {}", err);
        }
    };
}
