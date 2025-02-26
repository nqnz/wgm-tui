mod wg;
mod qr;
mod tui;
mod load_config;
mod config; 

use wg::generate_wg_config;
use qr::generate_qr_code;
use tui::start_tui;
use load_config::load_config;
use anyhow::{Result};

fn main() -> Result<()> {
    // Load the config from "config.toml"
    let config = load_config("config.toml")?;
    println!("Loaded config: {:?}", config);

    // Start interactive TUI and get user input
    let (client_ip, client_name) = start_tui()?;

    if client_ip.is_empty() || client_name.is_empty() {
        println!("❌ No input provided. Exiting.");
        return Ok(());
    }

    // Generate WireGuard config
    let config = generate_wg_config(&config, &client_name);

    // Generate QR code
    generate_qr_code(&config);

    println!("✅ Configuration successfully created for {client_name} with IP {client_ip}!");

    Ok(())
}