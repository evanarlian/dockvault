use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateEntry {
    auth_b64: String,
    username: String,
    is_used: bool,
}
impl StateEntry {
    pub fn from(username: String, is_used: bool, auth_b64: String) -> Self {
        StateEntry {
            username,
            is_used,
            auth_b64,
        }
    }
}

#[derive(Debug)]
pub struct State(BTreeMap<String, BTreeSet<StateEntry>>);
impl State {
    pub fn from(state: BTreeMap<String, BTreeSet<StateEntry>>) -> Self {
        State(state)
    }
}
