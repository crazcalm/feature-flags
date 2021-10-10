use clap::{App, Arg, SubCommand};

fn is_bool_validator(item: String) -> Result<(), String> {
    match item.as_str().trim().to_lowercase().as_str() {
        "true" => Ok(()),
        "false" => Ok(()),
        _ => Err("Must be 'true' or 'false'".to_string()),
    }
}

pub fn get_app() -> App<'static, 'static> {
    App::new("Feature Flags Program")
        .version("0.1.0")
        .author("Marcus Willock <crazcalm@gmail.com>")
        .about("Fill in later")
        .subcommands(vec![
            SubCommand::with_name("all").about("Print all flags to screen"),
            SubCommand::with_name("create")
                .about("Create a new flag")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .help("Name of the flag"),
                )
                .arg(
                    Arg::with_name("bool")
                        .required(true)
                        .validator(is_bool_validator)
                        .help("Bool"),
                ),
            SubCommand::with_name("update")
                .about("Update a flag by name")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .help("Name of the flag"),
                )
                .arg(
                    Arg::with_name("bool")
                        .required(true)
                        .validator(is_bool_validator)
                        .help("Bool"),
                ),
            SubCommand::with_name("get").about("Get flag by name").arg(
                Arg::with_name("name")
                    .required(true)
                    .help("Name of the flag"),
            ),
            SubCommand::with_name("delete")
                .about("Delete flag by name")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .help("Name of the flag"),
                ),
        ])
}

#[cfg(test)]
mod tests {
    use super::{get_app, is_bool_validator};

    #[test]
    fn test_is_bool_validator() {
        let inputs = vec![
            "True".to_string(),
            "  True  ".to_string(),
            "true".to_string(),
            "  true  ".to_string(),
            "False".to_string(),
            "  False  ".to_string(),
            "false".to_string(),
            "  false  ".to_string(),
        ];

        for case in inputs {
            assert_eq!(is_bool_validator(case), Ok(()));
        }
    }

    #[test]
    fn test_is_bool_validator_errors() {
        let inputs = vec!["".to_string(), "not a bool".to_string()];

        for case in inputs {
            assert_eq!(
                is_bool_validator(case),
                Err("Must be 'true' or 'false'".to_string()),
            );
        }
    }

    #[test]
    fn test_all_command() {
        let matches = get_app().get_matches_from(vec!["my_prog", "all"]);

        assert_eq!(true, matches.subcommand_matches("all").is_some())
    }

    #[test]
    fn test_get_command() -> Result<(), String> {
        let matches = get_app().get_matches_from(vec!["my_prog", "get", "test_name"]);

        if let Some(sub_get) = matches.subcommand_matches("get") {
            assert_eq!(Some("test_name"), sub_get.value_of("name"));
            Ok(())
        } else {
            Err(String::from("Check was not ran"))
        }
    }

    #[test]
    fn test_delete_command() -> Result<(), String> {
        let matches = get_app().get_matches_from(vec!["my_prog", "delete", "test_name"]);

        if let Some(sub_delete) = matches.subcommand_matches("delete") {
            assert_eq!(Some("test_name"), sub_delete.value_of("name"));
            Ok(())
        } else {
            Err(String::from("Check was not ran"))
        }
    }

    #[test]
    fn test_create_command() {
        let cases = vec![
            vec!["my_prog", "create", "test_name", "true"],
            vec!["my_prog", "create", "test_name", "false"],
        ];

        for case in cases {
            let matches = get_app().get_matches_from(case);

            if let Some(sub_create) = matches.subcommand_matches("create") {
                assert_eq!(Some("test_name"), sub_create.value_of("name"));
                assert_eq!(true, sub_create.value_of("bool").is_some());
            }
        }
    }

    #[test]
    fn test_create_command_errors() {
        let cases = vec![
            vec!["my_prog", "create"],
            vec!["my_prog", "create", "test_name"],
            vec!["my_prog", "create", "test_name", "Not a bool"],
        ];

        for case in cases {
            let matches = get_app().get_matches_from_safe(case);
            assert_eq!(matches.is_err(), true);
        }
    }

    #[test]
    fn test_update_command() {
        let cases = vec![
            vec!["my_prog", "update", "test_name", "true"],
            vec!["my_prog", "update", "test_name", "false"],
        ];

        for case in cases {
            let matches = get_app().get_matches_from(case);

            if let Some(sub_update) = matches.subcommand_matches("update") {
                assert_eq!(Some("test_name"), sub_update.value_of("name"));
                assert_eq!(true, sub_update.value_of("bool").is_some());
            }
        }
    }

    #[test]
    fn test_update_command_errors() {
        let cases = vec![
            vec!["my_prog", "update"],
            vec!["my_prog", "update", "test_name"],
            vec!["my_prog", "update", "test_name", "Not a bool"],
        ];

        for case in cases {
            let matches = get_app().get_matches_from_safe(case);
            assert_eq!(matches.is_err(), true);
        }
    }
}
