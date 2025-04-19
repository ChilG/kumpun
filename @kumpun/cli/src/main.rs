mod kumpun;

use crate::kumpun::args::cli::{Cli, Commands};
use crate::kumpun::commands;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    commands::init_all();

    match &cli.command {
        Commands::Dev => kumpun::commands::dev::run(),
        Commands::Check => kumpun::commands::check::run(),
        Commands::Docs => kumpun::commands::docs::run(),
        Commands::Generate {
            schema,
            target,
            schema_dir,
            out_dir,
        } => commands::generate::run(schema, target, schema_dir, out_dir),
    }
}
