use crate::parser::DockerConfig;
use crate::parser::DockvaultConfig;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::collections::{BTreeMap, BTreeSet};
use std::str;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateEntry {
    username: String,
    is_used: bool,
    auth_b64: String,
}

fn get_username_from_auth(auth_b64: &str) -> Option<String> {
    let decoded = BASE64_STANDARD.decode(auth_b64).ok()?;
    let decoded = str::from_utf8(&decoded).to_owned().ok()?;
    let (username, _password) = decoded.split_once(':')?;
    Some(username.to_string())
}

#[derive(Debug)]
pub struct State<'a> {
    docker_cfg: &'a mut DockerConfig,
    dockvault_cfg: &'a DockvaultConfig,
    entries: BTreeMap<String, BTreeSet<StateEntry>>,
}
impl<'a> State<'a> {
    pub fn make_state(
        docker_cfg: &'a mut DockerConfig,
        dockvault_cfg: &'a DockvaultConfig,
    ) -> Self {
        let mut entries = BTreeMap::new();
        for (registry, auth_cfgs) in dockvault_cfg.data() {
            let state_entries = auth_cfgs
                .iter()
                .filter_map(|auth_cfg| {
                    let auth_b64 = auth_cfg.auth()?;
                    let username = get_username_from_auth(&auth_b64)?;
                    let is_used = match docker_cfg.auth_configs().get(registry) {
                        Some(docker_auth_cfg) => docker_auth_cfg == auth_cfg,
                        None => false,
                    };
                    Some(StateEntry {
                        username,
                        is_used,
                        auth_b64: auth_b64.to_string(),
                    })
                })
                .collect();
            entries.insert(registry.clone(), state_entries);
        }
        State {
            docker_cfg,
            dockvault_cfg,
            entries,
        }
    }
    pub fn print(&self) {
        for (i, (registry, entries)) in self.entries.iter().enumerate() {
            if i != 0 {
                println!();
            }
            println!("{}", registry);
            for entry in entries {
                let used_symbol = match entry.is_used {
                    true => "*",
                    false => " ",
                };
                println!("  {} {}", used_symbol, entry.username);
            }
        }
    }
}
