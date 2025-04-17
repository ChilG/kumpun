use clap::Parser;

mod kumpun {
    pub mod args;
    pub mod commands;
}

use kumpun::args::cli::{Cli, Commands};
use kumpun::commands;

fn main() {
    let cli = Cli::parse();
    
    commands::init_all();
    
    match &cli.command {
        Commands::Dev => kumpun::commands::dev::run(),
        Commands::Check => kumpun::commands::check::run(),
        Commands::Docs => kumpun::commands::docs::run(),
        Commands::Generate { schema, target } => {
            kumpun::commands::generate::run(schema, target)
        }
    }
}
