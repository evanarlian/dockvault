pub mod parser;
pub mod state;

use clap::Parser;
use parser::get_application_state;
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
    let docker_cfg = home.join(".docker/config.json");
    let dockvault_cfg = home.join(".docker/dockvault.json");

    // cli
    let cli = Cli::parse();
    match cli.subcommand {
        Subcommand::Delete => {
            match fs::remove_file(&dockvault_cfg) {
                Ok(()) => println!("Deleted {}", dockvault_cfg.to_string_lossy()),
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => println!(
                        "{} does not exist, skipping",
                        dockvault_cfg.to_string_lossy(),
                    ),
                    _ => return Err(Box::new(e)),
                },
            };
        }
        Subcommand::List => {
            let state = get_application_state(&docker_cfg, &dockvault_cfg)?;
            state.print();
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
