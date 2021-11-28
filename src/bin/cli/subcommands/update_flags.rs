use std::io::Write;

use feature_flags::db;

pub fn update_flag(conn: db::DBLocal, name: String, value: i32, mut writer: impl Write) {
    let result = db::update_flag(conn, name, value);

    match result {
        Ok(_) => writer
            .write_all("Successfully updated the db\n".as_bytes())
            .unwrap(),
        Err(err) => writer
            .write_all(format!("Failed to add to the db: {:?}", err).as_bytes())
            .unwrap(),
    }
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

        db::initialize_db(local_conn.clone()).unwrap();

        local_conn
    }

    #[test]
    fn test_update_flag() {
        let conn = in_memory_db();

        let mut buffer = [0u8; 28];
        let buf_writer = BufWriter::new(buffer.as_mut());

        // add flag to db
        let _ = db::add_flag(conn.clone(), "test".to_string(), 0);

        let _ = update_flag(conn.clone(), "test".to_string(), 1, buf_writer);

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            "Successfully updated the db\n"
        );
    }
}
