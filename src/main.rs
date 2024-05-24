pub mod parser;
pub mod state;

use clap::Parser;
use std::{error::Error, fs, io::ErrorKind};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser, Debug, Clone)]
enum Subcommand {
    List,
    Delete,
    Use { registry: String, username: String },
}

fn main() -> Result<(), Box<dyn Error>> {
    // variables
    let home = dirs::home_dir().ok_or("Cannot get home directory")?;
    let docker_cfg_path = home.join(".docker/config.json");
    let dockvault_cfg_path = home.join(".docker/dockvault.json");

    // cli
    let cli = Cli::parse();
    match cli.subcommand {
        Subcommand::Delete => {
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
        Subcommand::List => {
            let (docker_cfg, dockvault_cfg) =
                parser::parse_and_merge(&docker_cfg_path, &dockvault_cfg_path)?;
            dbg!(&docker_cfg);
            dbg!(&dockvault_cfg);
            parser::save_cfg_file(&docker_cfg_path, &docker_cfg)?;
            parser::save_cfg_file(&dockvault_cfg_path, &dockvault_cfg)?;
            // state.print();
            println!("done saving");
        }
        Subcommand::Use { registry, username } => {
            println!(
                "Updating main docker config to use {} {}",
                registry, username
            );
        }
    }
    Ok(())
}

// TODO:
// save
// colored output (must adhere to )
// confirmation (interactive)
// shell output, just fish for now
// better error message
// file.go in cli, gold mine
