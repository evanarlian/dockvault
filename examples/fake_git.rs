use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    git_subcommand: GitSubcommand,
}

#[derive(Subcommand, Debug)]
enum GitSubcommand {
    Clone { repo: String },
    Commit(GitCommitArgs),
}

#[derive(Parser, Debug)]
struct GitCommitArgs {
    #[arg(short)]
    message: String,
}

fn main() {
    let cli = Cli::parse();
    dbg!(cli);
}
