mod ip_info;

use clap::Parser;
use ip_info::IPInfo;

#[derive(Parser, Debug)]
#[command(
    name = "what-the-ip",
    about = "Fetch IP address information.",
    long_about = None,
    version
)]
struct Args {
    /// IP address to look up (defaults to your own IP)
    #[arg(short = 'a', long, value_name = "IP")]
    ip: Option<String>,

    /// Output in JSON format (default = false)
    #[arg(short, long, default_value_t = false)]
    json: bool,

    /// Timeout in seconds (default = 10)
    #[arg(short, long, default_value_t = 10)]
    timeout: u64,

    /// Proxy server to use for the request
    #[arg(short, long, value_name = "PROXY")]
    proxy: Option<String>,

    /// Include extra metadata in the output (default = false)
    #[arg(short, long, default_value_t = false)]
    extra_metadata: bool,
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
