use feature_flags::db::{get_all_flags, DBLocal};

pub fn all_flags(db: DBLocal) {
    let rows = get_all_flags(db.clone()).expect("Unable to get all flags");
    for flag in rows {
        println!("flag: {}: {}", flag.name, flag.value);
    }
    println!("Done");
}
