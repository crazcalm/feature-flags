use std::io::Write;

use feature_flags::db::{get_flag_by_name, DBLocal};

pub fn get_flag(db: DBLocal, name: String, mut writer: impl Write) {
    let result = get_flag_by_name(db, name);
    match result {
        Ok(flag) => writer
            .write_all(format!("Flag -- {}: {}\n", flag.name, flag.value).as_bytes())
            .unwrap(),
        Err(_) => writer.write_all("No Flag Found\n".as_bytes()).unwrap(),
    };
}

#[cfg(test)]
mod tests {
    use std::io::BufWriter;
    use std::rc::Rc;

    use rusqlite::Connection;

    use feature_flags::db;

    use super::*;

    fn in_memory_db() -> db::DBLocal {
        let conn = Connection::open_in_memory().unwrap();

        let local_conn = Rc::new(conn);

        db::initialize_db(local_conn.clone());

        local_conn
    }

    #[test]
    fn test_get_flag() {
        let conn = in_memory_db();

        let mut buffer = [0u8; 20];
        let buf_writer = BufWriter::new(buffer.as_mut());

        // add flag to db
        let _ = db::add_flag(conn.clone(), "test".to_string(), 0);

        let _ = get_flag(conn.clone(), "test".to_string(), buf_writer);

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            "Flag -- test: false\n"
        );
    }
}
