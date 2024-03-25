use clap::Parser;
use omni_node::{client, server};
use serde::Serialize;
use std::net::IpAddr;

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum Mode {
    /// Mode of operation as client (default).
    #[default]
    Client,
    /// Mode of operation as server.
    Server,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Operation mode.
    #[arg(short, long, value_enum)]
    mode: Option<Mode>,

    /// Server IP address.
    #[arg(short, long, default_value = "127.0.0.1")]
    ip_addr_server: Option<IpAddr>,

    /// Server TCP port.
    #[arg(short, long, default_value = "9696")]
    port_server: Option<u16>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set up tracing subscribers.
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Parse command line arguments.
    let cli = Cli::parse();

    match cli.mode {
        Some(Mode::Server) => {
            server::start(cli.ip_addr_server, cli.port_server).await?;
        }
        _ => {
            client::start(cli.ip_addr_server, cli.port_server).await?;
        }
    }

    Ok(())
}
