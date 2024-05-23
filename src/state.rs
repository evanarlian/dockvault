use std::collections::BTreeMap;

#[derive(Debug)]
pub struct StateEntry {
    auth_b64: String,
    username: String,
    is_used: bool,
}
impl StateEntry {
    pub fn from(auth_b64: String, username: String, is_used: bool) -> Self {
        StateEntry {
            auth_b64,
            username,
            is_used,
        }
    }
}

#[derive(Debug)]
pub struct State(BTreeMap<String, Vec<StateEntry>>);
impl State {
    pub fn from(state: BTreeMap<String, Vec<StateEntry>>) -> Self {
        State(state)
    }
}
