mod wg;
mod qr;
mod tui;

use wg::generate_wg_config;
use qr::generate_qr_code;
use tui::start_tui;

fn main() -> std::io::Result<()> {
    let private_key = "CLIENT_PRIVATE_KEY";
    let public_key = "SERVER_PUBLIC_KEY";
    let server_ip = "192.168.1.1";
    let dns = "8.8.8.8";
    let allowed_ips = "0.0.0.0/0";

    // Start interactive TUI and get user input
    let (client_ip, client_name) = start_tui()?;

    if client_ip.is_empty() || client_name.is_empty() {
        println!("❌ No input provided. Exiting.");
        return Ok(());
    }

    // Generate WireGuard config
    let config = generate_wg_config(&client_name, private_key, public_key, server_ip, dns, allowed_ips);

    // Generate QR code
    generate_qr_code(&config);

    println!("✅ Configuration successfully created for {client_name} with IP {client_ip}!");

    Ok(())
}
