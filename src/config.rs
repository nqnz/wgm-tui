//use anyhow::Result;
use serde::Deserialize;
//use std::fs;
//use std::path::Path;
//use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_ip: String,
    pub server_port: u16,
    pub server_private_key: String,
    pub server_public_key: String,
    pub client_private_key: String,

    #[serde(default = "default_client_address")]
    pub client_address: String,

    #[serde(default = "default_dns")]
    pub dns: String,

    #[serde(default = "default_allowed_ips")]
    pub allowed_ips: String,

    #[serde(default = "default_keepalive")]
    pub persistent_keepalive: u16,
}

//defaults for optional fields
fn default_client_address() -> String {
    "10.0.0.2/32".to_string()
}
fn default_dns() -> String {
    "8.8.8.8".to_string()
}
fn default_allowed_ips() -> String {
    "0.0.0.0/0".to_string()
}
fn default_keepalive() -> u16 {
    25
}

