pub mod parser;
pub mod state;

use clap::{Parser, Subcommand, ValueEnum};
use std::error::Error;
use std::fs;
use std::io::ErrorKind;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    subcommand: DockvaultSubcommand,
}

#[derive(Subcommand, Debug)]
enum DockvaultSubcommand {
    #[command(about = "Stores docker config to dockvault.")]
    Merge,
    #[command(about = "Lists all users grouped by docker repository.")]
    List,
    #[command(about = "Deletes dockvault config data.")]
    Delete,
    #[command(about = "Change authenticated user for specified docker repo.")]
    Use { use_syntax: String },
    #[command(about = "Generate shell completions.")]
    Shell {
        #[arg(value_enum)]
        shell: Shell,
    },
    #[clap(hide = true)]
    Completion,
}

#[derive(Clone, Debug, ValueEnum)]
enum Shell {
    Fish,
}

fn main() -> Result<(), Box<dyn Error>> {
    // variables
    let home = dirs::home_dir().ok_or("Cannot get home directory")?;
    let docker_cfg_path = home.join(".docker/config.json");
    let dockvault_cfg_path = home.join(".dockvault/config.json");

    // cli
    let cli = Cli::parse();
    match cli.subcommand {
        DockvaultSubcommand::Merge => {
            let (_, dockvault_cfg) =
                parser::parse_and_merge(&docker_cfg_path, &dockvault_cfg_path)?;
            parser::save_cfg_file(&dockvault_cfg_path, &dockvault_cfg)?;
            println!(
                "Merged all `{}` to `{}`",
                docker_cfg_path.to_string_lossy(),
                dockvault_cfg_path.to_string_lossy()
            )
        }
        DockvaultSubcommand::Delete => {
            match fs::remove_file(&dockvault_cfg_path) {
                Ok(()) => println!("Deleted {}", dockvault_cfg_path.to_string_lossy()),
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => println!(
                        "{} does not exist, skipping",
                        dockvault_cfg_path.to_string_lossy(),
                    ),
                    _ => return Err(Box::new(e)),
                },
            };
        }
        DockvaultSubcommand::List => {
            let (docker_cfg, dockvault_cfg) =
                parser::parse_and_merge(&docker_cfg_path, &dockvault_cfg_path)?;
            parser::save_cfg_file(&dockvault_cfg_path, &dockvault_cfg)?;
            let state = state::make_state(&docker_cfg, &dockvault_cfg);
            state::print(&state);
        }
        DockvaultSubcommand::Use { use_syntax } => {
            let (mut docker_cfg, dockvault_cfg) =
                parser::parse_and_merge(&docker_cfg_path, &dockvault_cfg_path)?;
            parser::save_cfg_file(&dockvault_cfg_path, &dockvault_cfg)?;
            let state = state::make_state(&docker_cfg, &dockvault_cfg);
            let (username, registry) = use_syntax
                .split_once('@')
                .ok_or(format!("invalid use syntax: `{}`", use_syntax))?;
            state::change_who(&state, &mut docker_cfg, registry, username)?;
            parser::save_cfg_file(&docker_cfg_path, &docker_cfg)?;
            println!(
                "Updated docker config to use `{}` with username `{}`",
                registry, username
            );
        }
        DockvaultSubcommand::Shell { shell } => match shell {
            Shell::Fish => {
                let fish_completions = include_str!("../completions/dockvault.fish");
                println!("{}", fish_completions);
            }
        },
        DockvaultSubcommand::Completion => {
            let (docker_cfg, dockvault_cfg) =
                parser::parse_and_merge(&docker_cfg_path, &dockvault_cfg_path)?;
            parser::save_cfg_file(&dockvault_cfg_path, &dockvault_cfg)?;
            let state = state::make_state(&docker_cfg, &dockvault_cfg);
            state::generate_autocomplete(&state);
        }
    }
    Ok(())
}

// TODO:
// confirmation (interactive) when deleting, or when using
// better error message
