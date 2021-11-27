use std::io::Write;

use feature_flags::db::{get_all_flags, DBLocal};

pub fn all_flags_v2(db: DBLocal, mut writer: impl Write) {
    let rows = get_all_flags(db).expect("Unable to get all flags");
    for flag in rows {
        writer
            .write_all(format!("flag: {}: {}\n", flag.name, flag.value).as_bytes())
            .unwrap();
    }
    writer.write_all("Done\n".as_bytes()).unwrap();
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
    fn test_all_flags() {
        let conn = in_memory_db();

        let mut buffer = [0u8; 5];
        let buf_writer = BufWriter::new(buffer.as_mut());

        // Case: Zero Flags
        let _ = all_flags_v2(conn.clone(), buf_writer);

        assert_eq!(std::str::from_utf8(&buffer).unwrap(), "Done\n");

        // Case: More than Zero Flags
        let flags = vec![
            ("test_1".to_string(), 0),
            ("test_2".to_string(), 1),
            ("test_3".to_string(), 0),
        ];

        for (name, value) in flags {
            let _ = db::add_flag(conn.clone(), name, value).unwrap();
        }

        let mut buffer = [0u8; 64];
        let buf_writer = BufWriter::new(buffer.as_mut());

        let _ = all_flags_v2(conn.clone(), buf_writer);

        assert_eq!(
            std::str::from_utf8(&buffer).unwrap(),
            "flag: test_1: false\nflag: test_2: true\nflag: test_3: false\nDone\n"
        );
    }
}
