use std::io::Write;

use feature_flags::db::{add_flag, DBLocal};

pub fn create_flag(db: DBLocal, name: String, value: i32, mut writer: impl Write) {
    let result = add_flag(db, name, value);

    match result {
        Ok(_) => writer
            .write_all("Successfully added to the db\n".as_bytes())
            .unwrap(),
        Err(err) => writer
            .write_all(format!("Failed to add to db: {:?}", err).as_bytes())
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

        db::initialize_db(local_conn.clone());

        local_conn
    }

    #[test]
    fn test_create_flag() {
        let conn = in_memory_db();

        let mut buffer = [0u8; 29];
        let buf_writer = BufWriter::new(buffer.as_mut());

        let _ = create_flag(conn.clone(), "test".to_string(), 0, buf_writer);

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            "Successfully added to the db\n"
        );
    }
}
