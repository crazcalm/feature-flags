use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Create(CreateArgs),
    Update(UpdateArgs),
    Get(GetArgs),
    Delete(DeleteArgs),
}

#[derive(Args, Debug)]
pub struct CreateArgs {
    /// Flag Name
    pub name: String,
}

#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// Flag Name
    #[arg(short, long, required = true)]
    pub name: String,
    /// Flag Value
    #[arg(short, long, required = true, help = "flag value")]
    pub value: Option<bool>,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct GetArgs {
    /// Flag Name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Show All Flags
    #[arg(short, long)]
    pub all: bool,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// Flag name
    pub name: String,
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{Cli, Commands};

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }

    #[test]
    fn test_get_command() {
        let input = vec!["my_prog", "get", "-n", "test_name"];
        let cli = Cli::parse_from(input.clone());

        match cli.command {
            Commands::Get(get) => {
                assert_eq!("test_name", get.name.unwrap(), "Failed input: {:?}", input);
            }
            _ => panic!("Get subcommand was not called"),
        }
    }

    #[test]
    fn test_get_all_command() {
        let input = vec!["my_prog", "get", "-a"];
        let cli = Cli::parse_from(input.clone());

        match cli.command {
            Commands::Get(get) => {
                assert_eq!(true, get.all, "Failed input: {:?}", input);
            }
            _ => panic!("Get subcommand was not called"),
        }
    }

    #[test]
    fn test_delete_command() {
        let input = vec!["my_prog", "delete", "test_name"];
        let cli = Cli::parse_from(input.clone());

        match &cli.command {
            Commands::Delete(delete) => {
                assert_eq!("test_name", delete.name, "Failed input: {:?}", input);
            }
            _ => panic!("Delete subcommand was not called"),
        }
    }

    #[test]
    fn test_create_command() {
        let cases = vec![
            vec!["my_prog", "create", "test_name"],
            vec!["my_prog", "create", "test_name"],
        ];

        for case in cases {
            let cli = Cli::parse_from(case.clone());

            match &cli.command {
                Commands::Create(create) => {
                    assert_eq!(case[2], create.name);
                }
                _ => panic!("Case failed: {:?}", case),
            }
        }
    }

    #[test]
    fn test_update_command() {
        let cases = vec![
            vec!["my_prog", "update", "-n", "test_name", "-v", "true"],
            vec!["my_prog", "update", "-n", "test_name", "-v", "false"],
        ];

        for case in cases {
            let cli = Cli::parse_from(case.clone());

            match cli.command {
                Commands::Update(update) => {
                    assert_eq!(case[3], update.name, "Failed case: {:?}", case);
                    assert_eq!(
                        case[5],
                        update.value.unwrap().to_string(),
                        "Failed case: {:?}",
                        case
                    );
                }
                _ => panic!("Case failed: {:?}", case),
            }
        }
    }
}
