use crate::parser::DockerData;
use crate::parser::DockvaultData;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateEntry {
    username: String,
    is_used: bool,
    auth_b64: String,
}

#[derive(Debug)]
pub struct State(BTreeMap<String, BTreeSet<StateEntry>>);
impl State {
    pub fn print(&self) {
        for (i, (registry, entries)) in self.0.iter().enumerate() {
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
    pub fn make_state(
        docker_data: &DockerData,
        dockvault_data: &DockvaultData,
    ) -> Result<State, Box<dyn Error>> {
        let mut state_map = BTreeMap::new();
        // add all dockvault data to state map
        for (registry, cred_entries) in dockvault_data.data() {
            let state_entries: BTreeSet<StateEntry> = cred_entries
                .iter()
                .filter_map(|c| c.decode().ok())
                .map(|c| StateEntry {
                    username: c.username().to_owned(),
                    is_used: match docker_data.auths().get(registry) {
                        Some(ce) => ce.auth_b64() == c.auth(),
                        None => false,
                    },
                    auth_b64: c.auth().to_owned(),
                })
                .collect();
            state_map.insert(registry.clone(), state_entries);
        }
        // add all docker data to state map, there might be new stuffs
        for (registry, cred_entry) in docker_data.auths() {
            let state_entries = state_map.entry(registry.clone()).or_default();
            let decoded = cred_entry.decode()?;
            state_entries.insert(StateEntry {
                username: decoded.username().to_owned(),
                is_used: true,
                auth_b64: decoded.auth().to_owned(),
            });
        }
        Ok(State(state_map))
    }
}
