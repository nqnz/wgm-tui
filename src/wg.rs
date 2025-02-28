use std::fs::File;
use std::io::Write;
use crate::config::Config;

pub fn generate_wg_config(cfg: &Config, client_name: &str) -> String {
    let config = format!(
        "[Interface]
Address = 10.0.0.2/24
DNS = {dns}
PrivateKey = {client_private_key}

[Peer]
PublicKey = {server_public_key}
Endpoint = {server_ip}:51820
AllowedIPs = {allowed_ips} 
PersistentKeepalive = 25",
        
        client_private_key = cfg.client_private_key,
        dns = cfg.dns,
        server_public_key = cfg.server_public_key,
        server_ip = cfg.server_ip,
        allowed_ips = cfg.allowed_ips,
    );

    let filename = format!("{}.conf", client_name);
    let mut file = File::create(&filename).expect("Failed to create file");
    file.write_all(config.as_bytes()).expect("Failed to write config");

    println!("✅ WireGuard configuration saved to: {}", filename);
    config
}
