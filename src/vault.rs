use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use serde::Deserialize;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
struct DockerAuths(HashMap<String, CredEntry>);

#[derive(Debug)]
struct Args {
    name: String,
    age: i32,
}

fn main() {
    println!("im on example");
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
struct DockerAuthData(HashMap<String, CredEntry>);

#[derive(Serialize, Deserialize, Debug)]
struct DockerData {
    auths: DockerAuthData,
    raw: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct DockvaultData(HashMap<String, Vec<CredEntry>>);
impl DockvaultData {
    fn new() -> Self {
        DockvaultData(HashMap::new())
    }
}

#[derive(Serialize, Deserialize, Debug)]

pub struct State {
    docker_data: DockerData,
    dockvault_data: DockvaultData,
}

pub fn get_application_state(
    docker_cfg: &Path,
    dockvault_cfg: &Path,
) -> Result<State, Box<dyn Error>> {
    let docker_data = parse_docker_cfg(docker_cfg)?;
    let dockvault_data = parse_dockvault_cfg(dockvault_cfg)?;
    Ok(State {
        docker_data,
        dockvault_data,
    })
}

fn parse_docker_cfg(docker_cfg: &Path) -> Result<DockerData, Box<dyn Error>> {
    let content = match fs::read_to_string(docker_cfg) {
        Ok(content) => content,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => String::from("{}"),
            _ => return Err(Box::new(e)),
        },
    };
    let json: serde_json::Value = serde_json::from_str(&content)?;
    let raw_auths = json
        .get("auths")
        .ok_or(format!(
            "{} is not dict-like.",
            docker_cfg.to_string_lossy()
        ))?
        .clone();
    let auths: DockerAuthData = serde_json::from_value(raw_auths)?;
    Ok(DockerData { auths, raw: json })
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
