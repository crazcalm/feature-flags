use std::io::Write;

use feature_flags::db::{delete_flag_by_name, DBLocal};

pub fn delete_flag(db: DBLocal, name: String, mut writer: impl Write) {
    let result = delete_flag_by_name(db, name);
    match result {
        Ok(deleted) => {
            writer
                .write_all(format!("{} row deleted\n", deleted).as_bytes())
                .unwrap();
        }
        Err(err) => {
            writer
                .write_all(format!("delete failed: {:?}\n", err).as_bytes())
                .unwrap();
        }
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
    fn test_delete_flag() {
        let conn = in_memory_db();

        let mut buffer = [0u8; 14];
        let buf_writer = BufWriter::new(buffer.as_mut());

        // add flag to db
        let _ = db::add_flag(conn.clone(), "test".to_string(), 0);

        let _ = delete_flag(conn.clone(), "test".to_string(), buf_writer);

        assert_eq!(std::str::from_utf8(&buffer).unwrap(), "1 row deleted\n");
    }

    #[test]
    fn test_delete_flag_zero_rows_deleted() {
        let conn = in_memory_db();

        let mut buffer = [0u8; 14];
        let buf_writer = BufWriter::new(buffer.as_mut());

        let _ = delete_flag(conn.clone(), "test".to_string(), buf_writer);

        assert_eq!(std::str::from_utf8(&buffer).unwrap(), "0 row deleted\n");
    }
}
