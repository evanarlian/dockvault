use crate::parser::{AuthConfig, DockerConfig, DockvaultConfig};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::collections::BTreeMap;
use std::error::Error;
use std::str;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateEntry<'a> {
    is_used: bool,
    auth_b64: String,
    auth_cfg: &'a AuthConfig,
}

pub struct State<'a>(BTreeMap<String, BTreeMap<String, StateEntry<'a>>>);

fn get_username_from_auth(auth_b64: &str) -> Option<String> {
    let decoded = BASE64_STANDARD.decode(auth_b64).ok()?;
    let decoded = str::from_utf8(&decoded).to_owned().ok()?;
    let (username, _password) = decoded.split_once(':')?;
    Some(username.to_string())
}

pub fn make_state<'a>(
    docker_cfg: &DockerConfig,
    dockvault_cfg: &'a DockvaultConfig,
) -> State<'a> {
    let mut entries = BTreeMap::new();
    for (registry, auth_cfgs) in dockvault_cfg.data() {
        let state_entries = auth_cfgs
            .iter()
            .filter_map(|auth_cfg| {
                let auth_b64 = auth_cfg.auth()?;
                let username = get_username_from_auth(auth_b64)?;
                let is_used = match docker_cfg.auth_configs().get(registry) {
                    Some(docker_auth_cfg) => docker_auth_cfg == auth_cfg,
                    None => false,
                };
                let state_entry = StateEntry {
                    is_used,
                    auth_b64: auth_b64.to_string(),
                    auth_cfg,
                };
                Some((username, state_entry))
            })
            .collect();
        entries.insert(registry.clone(), state_entries);
    }
    State(entries)
}

pub fn print(state: &State) {
    if state.0.is_empty() {
        println!("You haven't logged in to any registry.");
        return;
    }
    for (i, (registry, entries)) in state.0.iter().enumerate() {
        if i != 0 {
            println!();
        }
        println!("{}", registry);
        for (username, state_entry) in entries {
            let used_symbol = match state_entry.is_used {
                true => "*",
                false => " ",
            };
            println!("  {} {}", used_symbol, username);
        }
    }
}

pub fn change_who(
    state: &State,
    docker_cfg: &mut DockerConfig,
    registry: &str,
    username: &str,
) -> Result<(), Box<dyn Error>> {
    let users_mapping = state
        .0
        .get(registry)
        .ok_or(format!("registry `{}` does not exist.", registry))?;
    let current_user_state = users_mapping.get(username).ok_or(format!(
        "username `{}` does not exist in registry {}.",
        username, registry
    ))?;
    docker_cfg.change_auth_cfg(registry.to_string(), current_user_state.auth_cfg.clone());
    Ok(())
}

pub fn generate_autocomplete(state: &State) {
    for (registry, entries) in state.0.iter() {
        for username in entries.keys() {
            println!("{}@{}", username, registry);
        }
    }
}
