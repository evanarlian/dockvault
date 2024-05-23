use serde::Deserialize;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct CredEntry {
    auth: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DockerAuths(HashMap<String, CredEntry>);

fn main() -> Result<(), Box<dyn Error>> {
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
