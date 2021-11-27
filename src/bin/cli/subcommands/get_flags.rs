use feature_flags::db::{get_flag_by_name, DBLocal};

pub fn get_flag(db: DBLocal, name: String) {
    let result = get_flag_by_name(db, name);
    match result {
        Ok(flag) => {
            println!("Flag: {}: {}", flag.name, flag.value);
        }
        Err(_) => {
            println!("No Flag Found")
        }
    };
}
