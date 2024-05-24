use base64::Engine;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::str;

// https://github.com/docker/cli/blob/master/cli/config/types/authconfig.go
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct AuthConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serveraddress: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    identitytoken: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registrytoken: Option<String>,
}
impl AuthConfig {
    pub fn auth(&self) -> Option<&str> {
        (&self.auth).as_deref()
    }
}

// https://github.com/docker/cli/blob/master/cli/config/configfile/file.go
#[derive(Serialize, Deserialize, Debug)]
struct ProxyConfig {
    #[serde(rename = "httpProxy", skip_serializing_if = "Option::is_none")]
    http_proxy: Option<String>,
    #[serde(rename = "httpsProxy", skip_serializing_if = "Option::is_none")]
    https_proxy: Option<String>,
    #[serde(rename = "noProxy", skip_serializing_if = "Option::is_none")]
    no_proxy: Option<String>,
    #[serde(rename = "ftpProxy", skip_serializing_if = "Option::is_none")]
    ftp_proxy: Option<String>,
    #[serde(rename = "allProxy", skip_serializing_if = "Option::is_none")]
    all_proxy: Option<String>,
}

// https://github.com/docker/cli/blob/master/cli/config/configfile/file.go
#[derive(Serialize, Deserialize, Debug)]
pub struct DockerConfig {
    #[serde(default, rename = "auths")]
    auth_configs: BTreeMap<String, AuthConfig>,
    #[serde(rename = "HttpHeaders", skip_serializing_if = "Option::is_none")]
    http_headers: Option<BTreeMap<String, String>>,
    #[serde(rename = "psFormat", skip_serializing_if = "Option::is_none")]
    ps_format: Option<String>,
    #[serde(rename = "imagesFormat", skip_serializing_if = "Option::is_none")]
    images_format: Option<String>,
    #[serde(rename = "networksFormat", skip_serializing_if = "Option::is_none")]
    networks_format: Option<String>,
    #[serde(rename = "pluginsFormat", skip_serializing_if = "Option::is_none")]
    plugins_format: Option<String>,
    #[serde(rename = "volumesFormat", skip_serializing_if = "Option::is_none")]
    volumes_format: Option<String>,
    #[serde(rename = "statsFormat", skip_serializing_if = "Option::is_none")]
    stats_format: Option<String>,
    #[serde(rename = "detachKeys", skip_serializing_if = "Option::is_none")]
    detach_keys: Option<String>,
    #[serde(rename = "credsStore", skip_serializing_if = "Option::is_none")]
    credential_store: Option<String>,
    #[serde(rename = "credHelpers", skip_serializing_if = "Option::is_none")]
    credential_helpers: Option<BTreeMap<String, String>>,
    #[serde(
        rename = "serviceInspectFormat",
        skip_serializing_if = "Option::is_none"
    )]
    service_inspect_format: Option<String>,
    #[serde(rename = "servicesFormat", skip_serializing_if = "Option::is_none")]
    services_format: Option<String>,
    #[serde(rename = "tasksFormat", skip_serializing_if = "Option::is_none")]
    tasks_format: Option<String>,
    #[serde(rename = "secretFormat", skip_serializing_if = "Option::is_none")]
    secret_format: Option<String>,
    #[serde(rename = "configFormat", skip_serializing_if = "Option::is_none")]
    config_format: Option<String>,
    #[serde(rename = "nodesFormat", skip_serializing_if = "Option::is_none")]
    nodes_format: Option<String>,
    #[serde(rename = "pruneFilters", skip_serializing_if = "Option::is_none")]
    prune_filters: Option<Vec<String>>,
    #[serde(rename = "proxies", skip_serializing_if = "Option::is_none")]
    proxies: Option<BTreeMap<String, ProxyConfig>>,
    #[serde(rename = "experimental", skip_serializing_if = "Option::is_none")]
    experimental: Option<String>,
    #[serde(rename = "currentContext", skip_serializing_if = "Option::is_none")]
    current_context: Option<String>,
    #[serde(
        rename = "cliPluginsExtraDirs",
        skip_serializing_if = "Option::is_none"
    )]
    cli_plugins_extra_dirs: Option<Vec<String>>,
    #[serde(rename = "plugins", skip_serializing_if = "Option::is_none")]
    plugins: Option<BTreeMap<String, BTreeMap<String, String>>>,
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    aliases: Option<BTreeMap<String, String>>,
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    features: Option<BTreeMap<String, String>>,
}
impl DockerConfig {
    pub fn auth_configs(&self) -> &BTreeMap<String, AuthConfig> {
        &self.auth_configs
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockvaultConfig(BTreeMap<String, BTreeSet<AuthConfig>>);
impl DockvaultConfig {
    pub fn data(&self) -> &BTreeMap<String, BTreeSet<AuthConfig>> {
        &self.0
    }
}

pub fn save_cfg_file<T: Serialize>(cfg_path: &Path, cfg: &T) -> Result<(), Box<dyn Error>> {
    let mut buf = Vec::new();
    let formatter = PrettyFormatter::with_indent(b"\t");
    let mut serializer = serde_json::Serializer::with_formatter(&mut buf, formatter);
    let val = serde_json::to_value(cfg)?;
    val.serialize(&mut serializer);
    fs::write(cfg_path, buf)?;
    Ok(())
}

fn parse_cfg_file<T: DeserializeOwned>(cfg_file: &Path) -> Result<T, Box<dyn Error>> {
    // if file not found, user might never login before, don't error
    let content = match fs::read_to_string(cfg_file) {
        Ok(content) => content,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => String::from("{}"),
            _ => return Err(Box::new(e)),
        },
    };
    let config: T = serde_json::from_str(&content)?;
    Ok(config)
}

fn merge(docker_cfg: &DockerConfig, dockvault_cfg: &mut DockvaultConfig) {
    for (registry, auth_cfg) in &docker_cfg.auth_configs {
        let auths = dockvault_cfg.0.entry(registry.clone()).or_default();
        auths.insert(auth_cfg.clone());
    }
}

pub fn parse_and_merge(
    docker_cfg: &Path,
    dockvault_cfg: &Path,
) -> Result<(DockerConfig, DockvaultConfig), Box<dyn Error>> {
    let docker_cfg: DockerConfig = parse_cfg_file(docker_cfg)?;
    let mut dockvault_cfg: DockvaultConfig = parse_cfg_file(dockvault_cfg)?;
    merge(&docker_cfg, &mut dockvault_cfg);
    Ok((docker_cfg, dockvault_cfg))
}
