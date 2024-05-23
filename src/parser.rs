use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::Deserialize;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::StreamDeserializer;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::ErrorKind;
use std::path::Path;
use std::str;

use crate::state;
use crate::state::StateEntry;

#[derive(Serialize, Deserialize, Debug)]
struct DockerAuths(HashMap<String, CredEntry>);

#[derive(Serialize, Deserialize, Debug)]
struct CredEntry {
    auth: String,
}
impl CredEntry {
    fn decode(&self) -> Result<DecodedCred, Box<dyn Error>> {
        let decoded = str::from_utf8(&BASE64_STANDARD.decode(&self.auth)?)?.to_owned();
        let (username, _password) = decoded
            .split_once(':')
            .ok_or("unparsable base64-decoded string")?;
        Ok(DecodedCred {
            auth: self.auth.clone(),
            username: username.to_owned(),
        })
    }
}

struct DecodedCred {
    auth: String,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DockerData {
    auths: HashMap<String, CredEntry>,
    #[serde(flatten)]
    others: serde_json::Map<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DockvaultData(HashMap<String, Vec<CredEntry>>);

fn parse_docker_cfg(docker_cfg: &Path) -> Result<DockerData, Box<dyn Error>> {
    // if file not found, user might never login before, don't error
    let content = match fs::read_to_string(docker_cfg) {
        Ok(content) => content,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => String::from("{}"),
            _ => return Err(Box::new(e)),
        },
    };
    // if "auths" key not found, user might never login too, don't error
    let mut json: serde_json::Value = serde_json::from_str(&content)?;
    let map = json.as_object_mut().ok_or("json is not key-value pairs.")?;
    let raw_auths = map
        .entry("auths")
        .or_insert(serde_json::Value::from("{}"))
        .clone();
    // remove the original auths to prevetn double key
    map.remove("auths");
    // actually parse auths
    let auths = serde_json::from_value(raw_auths)?;
    Ok(DockerData {
        auths,
        others: map.clone(),
    })
}

fn parse_dockvault_cfg(dockvault_cfg: &Path) -> Result<DockvaultData, Box<dyn Error>> {
    let content = match fs::read_to_string(dockvault_cfg) {
        Ok(content) => content,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => String::from("{}"),
            _ => return Err(Box::new(e)),
        },
    };
    let dockvault_data = serde_json::from_str(&content)?;
    Ok(dockvault_data)
}

fn make_state(docker_data: &DockerData, dockvault_data: &DockvaultData) -> Result<state::State, Box<dyn Error>> {
    let mut state_map = BTreeMap::new();
    // add all dockvault data to state map
    for (registry, cred_entries) in &dockvault_data.0 {
        let state_entries: BTreeSet<StateEntry> = cred_entries
            .iter()
            .filter_map(|c| c.decode().ok())
            .map(|c| {
                state::StateEntry::from(
                    c.username,
                    match docker_data.auths.get(registry) {
                        Some(ce) => ce.auth == c.auth,
                        None => false,
                    },
                    c.auth.clone(),
                )
            })
            .collect();
        state_map.insert(registry.clone(), state_entries);
    }
    // add all docker data to state map, there might be new stuff
    for (registry, cred_entry) in &docker_data.auths {
        let state_entries = state_map.entry(registry.clone()).or_default();
        let decoded = cred_entry.decode()?;
        state_entries.insert(state::StateEntry::from(decoded.username, true, decoded.auth));
    }
    Ok(state::State::from(state_map))
}

pub fn get_application_state(
    docker_cfg: &Path,
    dockvault_cfg: &Path,
) -> Result<state::State, Box<dyn Error>> {
    let docker_data = parse_docker_cfg(docker_cfg)?;
    let dockvault_data = parse_dockvault_cfg(dockvault_cfg)?;
    let state = make_state(&docker_data, &dockvault_data)?;
    Ok(state)
}

fn main2() -> Result<(), Box<dyn Error>> {
    let home = dirs::home_dir().unwrap();
    let docker_config_path = home.join(".docker/config.json");
    let content = fs::read_to_string(docker_config_path)?;
    let mut parsed: serde_json::Value = serde_json::from_str(&content)?;
    let auths_value = parsed.get("auths").expect("must have auths key.").clone();
    let auths: HashMap<String, CredEntry> = serde_json::from_value(auths_value)?;
    println!("{:#?}", auths);
    println!("{:#?}", parsed);
    // put back
    let mut omut = parsed.as_object_mut().expect("json is not dict-shaped.");
    omut.insert(String::from("auths2"), serde_json::to_value(auths)?);
    println!("{:#?}", omut);

    // save to file
    let mut buf = Vec::new();
    let formatter = PrettyFormatter::with_indent(b"\t");
    let mut serializer = serde_json::Serializer::with_formatter(&mut buf, formatter);
    let obj_again = serde_json::to_value(omut).unwrap();
    obj_again.serialize(&mut serializer);
    let docker_config2_path = home.join(".docker/config2.json");
    fs::write(docker_config2_path, buf).expect("failed to write");

    Ok(())
}
