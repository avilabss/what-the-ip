mod ip_info;

use clap::Parser;
use ip_info::IPInfo;

#[derive(Parser, Debug)]
struct Args {
    // Optional IP address to look up
    #[arg(short, long)]
    ip: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let ip_info = IPInfo::fetch(args.ip.as_deref()).await;
    match ip_info {
        Ok(data) => data.print(),
        Err(e) => eprintln!("Error fetching IP info: {}", e),
    }
    Ok(())
}
