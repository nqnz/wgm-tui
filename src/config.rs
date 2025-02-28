//use anyhow::Result;
use serde::Deserialize;
//use std::fs;
//use std::path::Path;
//use toml;
use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::Ed25519KeyPair;

#[derive(Debug, Deserialize)]
pub struct Config {

    pub server_ip: String,

    #[serde(default = "default_server_port")]
    pub server_port: u16,

    //pub server_private_key: String,
    // we won't need the server private IP bc this code block should only be generating the client config
    pub server_public_key: String,

    #[serde(default = "generate_client_private_key")]
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

fn generate_client_private_key() -> String {
    let rng = SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng).expect("Failed to generate key pair");
    base64::encode(pkcs8_bytes.as_ref())
}

fn default_server_port() -> u16 {
    51820
}   
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

