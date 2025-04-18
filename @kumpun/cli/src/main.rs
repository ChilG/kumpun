use clap::Parser;

mod kumpun {
    pub mod args;
    pub mod commands;
}

use kumpun::args::cli::{Cli, Commands};
use kumpun::commands;

fn main() {
    // commands::generate::run(
    //     "everything.example",
    //     "rust",
    //     "cli/tests/fixtures/schemas",
    //     "cli/tests/generated",
    // );

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
