pub mod vault;

use clap::Parser;
use std::error::Error;
use vault::get_application_state;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Parser, Debug, Clone)]
enum Subcommand {
    List,
    Clear,
    Use { registry: String, username: String },
}

fn main() -> Result<(), Box<dyn Error>> {
    // variables
    let home = dirs::home_dir().ok_or("Cannot get home directory")?;
    let docker_cfg = home.join(".docker/config.json");
    let dockvault_cfg = home.join(".docker/dockvault.json");

    // state means the current status of both files
    let mut state = get_application_state(&docker_cfg, &dockvault_cfg)?;
    dbg!(state);
    // cli
    let cli = Cli::parse();
    match cli.subcommand {
        Subcommand::Clear => {
            println!("Deleted {:?}", dockvault_cfg);
        }
        Subcommand::List => {
            println!("Here is the list ");
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
