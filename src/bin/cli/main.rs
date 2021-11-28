use std::io;

mod cli;
mod subcommands;

use feature_flags::db::get_db_rc;

fn convert_string_to_sqlite_bool(value: &str) -> Option<i32> {
    match value.trim().to_lowercase().as_str() {
        "true" => Some(1),
        "false" => Some(0),
        _ => None,
    }
}

fn main() {
    let matches = cli::get_app().get_matches();

    match matches.subcommand {
        None => {
            eprintln!("TODO: Show cli help")
        }
        Some(command) => match command.name.as_ref() {
            "all" => {
                let db = get_db_rc();

                let stdout = io::stdout();
                let writer = stdout.lock();

                subcommands::all_flags::all_flags(db, writer);
            }
            "create" => {
                let name = command.matches.value_of("name").unwrap().to_string();
                let value =
                    convert_string_to_sqlite_bool(command.matches.value_of("bool").unwrap())
                        .unwrap();

                let stdout = io::stdout();
                let writer = stdout.lock();

                let db = get_db_rc();
                subcommands::create_flags::create_flag(db, name, value, writer)
            }
            "update" => {
                let name = command.matches.value_of("name").unwrap().to_string();
                let value =
                    convert_string_to_sqlite_bool(command.matches.value_of("bool").unwrap())
                        .unwrap();

                let db = get_db_rc();
                let stdout = io::stdout();
                let writer = stdout.lock();

                subcommands::update_flags::update_flag(db, name, value, writer)
            }
            "get" => {
                let name = command.matches.value_of("name").unwrap().to_string();

                let db = get_db_rc();
                let stdout = io::stdout();
                let writer = stdout.lock();

                subcommands::get_flags::get_flag(db, name, writer);
            }
            "delete" => {
                let name = command.matches.value_of("name").unwrap().to_string();
                let db = get_db_rc();
                let stdout = io::stdout();
                let writer = stdout.lock();

                subcommands::delete_flags::delete_flag(db, name, writer);
            }
            _ => panic!("A subcommand was added to the cli but was not connected to the cli"),
        },
    };
}

#[cfg(test)]
mod tests {
    use super::convert_string_to_sqlite_bool;

    #[test]
    fn test_convert_string_to_sqlite_bool() {
        let cases = vec![
            ("true", Some(1)),
            ("True", Some(1)),
            ("  True  ", Some(1)),
            ("false", Some(0)),
            ("False", Some(0)),
            ("  False  ", Some(0)),
            ("something else", None),
        ];

        for (input, expected) in cases {
            assert_eq!(convert_string_to_sqlite_bool(input), expected);
        }
    }
}
