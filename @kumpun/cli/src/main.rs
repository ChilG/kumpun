mod kumpun;

use crate::kumpun::args::cli::{Cli, Commands};
use crate::kumpun::commands;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    commands::init_all();

    match &cli.command {
        Commands::Dev => commands::dev::run(),
        Commands::Check => commands::check::run(),
        Commands::Docs => commands::docs::run(),
        Commands::Generate {
            schema,
            target,
            schema_dir,
            out_dir,
            with_docs,
            with_validation,
        } => commands::generate::run(
            schema,
            target,
            schema_dir,
            out_dir,
            with_docs,
            with_validation,
        ),
    }
}
