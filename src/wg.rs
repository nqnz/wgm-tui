use std::fs::File;
use std::io::Write;

pub fn generate_wg_config(cfg: &Config) -> String {
    let config = format!(
        "[Interface]
PrivateKey = {private_key}
Address = 10.0.0.2/24
DNS = {dns}

[Peer]
PublicKey = {server_public_key}
Endpoint = {server_ip}:51820
AllowedIPs = {allowed_ips} 
PersistentKeepalive = 25",
    );

    let filename = format!("{client_name}.conf");
    let mut file = File::create(&filename).expect("Failed to create file");
    file.write_all(config.as_bytes()).expect("Failed to write config");

    println!("✅ WireGuard configuration saved to: {}", filename);
    config
}
