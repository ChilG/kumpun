use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kumpun")]
#[command(about = "The Kumpun CLI 🐾")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Dev,
    Check,
    Docs,
    Generate {
        #[arg(short, long)]
        schema: String,

        #[arg(short, long, default_value = "rust")]
        target: String,

        #[arg(long, default_value = "schemas")]
        schema_dir: String,

        #[arg(long, default_value = "generated")]
        out_dir: String,

        #[arg(long)]
        with_docs: bool,

        #[arg(long)]
        with_validation: bool,
    },
}
