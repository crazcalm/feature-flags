use std::io;

use cli::{Cli, Commands};

mod cli;
mod subcommands;

use clap::Parser;
use feature_flags::db::get_db_rc;

fn convert_bool_to_sqlite_bool(value: bool) -> i32 {
    match value {
        true => 1,
        false => 0,
    }
}

fn main() {
    let cli_app = Cli::parse();

    let db = get_db_rc();
    let stdout = io::stdout();
    let writer = stdout.lock();

    match cli_app.command {
        Commands::Get(args) => {
            if let Some(name) = args.name {
                subcommands::get_flags::get_flag(db, name, writer);
            } else if args.all {
                subcommands::all_flags::all_flags(db, writer);
            }
        }
        Commands::Create(args) => {
            // All new flags are true
            let value = convert_bool_to_sqlite_bool(true);

            subcommands::create_flags::create_flag(db, args.name, value, writer);
        }
        Commands::Update(args) => {
            let name = args.name;
            let value = convert_bool_to_sqlite_bool(args.value.unwrap());

            subcommands::update_flags::update_flag(db, name, value, writer);
        }
        Commands::Delete(args) => {
            subcommands::delete_flags::delete_flag(db, args.name, writer);
        }
    };
}

#[cfg(test)]
mod tests {}
