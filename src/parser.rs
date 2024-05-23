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

use crate::state::State;

#[derive(Serialize, Deserialize, Debug)]
struct DockerAuths(HashMap<String, CredEntry>);

#[derive(Serialize, Deserialize, Debug)]
pub struct CredEntry {
    #[serde(rename = "auth")]
    auth_b64: String,
}
impl CredEntry {
    pub fn decode(&self) -> Result<DecodedCred, Box<dyn Error>> {
        let decoded = str::from_utf8(&BASE64_STANDARD.decode(&self.auth_b64)?)?.to_owned();
        let (username, _password) = decoded
            .split_once(':')
            .ok_or("unparsable base64-decoded string")?;
        Ok(DecodedCred {
            auth: self.auth_b64.clone(),
            username: username.to_owned(),
        })
    }
    pub fn auth_b64(&self) -> &str {
        &self.auth_b64
    }
}

pub struct DecodedCred {
    auth: String,
    username: String,
}
impl DecodedCred {
    pub fn auth(&self) -> &str {
        &self.auth
    }
    pub fn username(&self) -> &str {
        &self.username
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerData {
    auths: HashMap<String, CredEntry>,
    #[serde(flatten)]
    others: serde_json::Map<String, serde_json::Value>,
}
impl DockerData {
    pub fn auths(&self) -> &HashMap<String, CredEntry> {
        &self.auths
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockvaultData(HashMap<String, Vec<CredEntry>>);
impl DockvaultData {
    pub fn data(&self) -> &HashMap<String, Vec<CredEntry>> {
        &self.0
    }
}

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

pub fn get_application_state(
    docker_cfg: &Path,
    dockvault_cfg: &Path,
) -> Result<State, Box<dyn Error>> {
    let docker_data = parse_docker_cfg(docker_cfg)?;
    let dockvault_data = parse_dockvault_cfg(dockvault_cfg)?;
    let state = State::make_state(&docker_data, &dockvault_data)?;
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
