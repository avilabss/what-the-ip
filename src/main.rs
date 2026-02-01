mod ip_info;

use clap::Parser;
use ip_info::IPInfoClient;

#[derive(Parser, Debug)]
#[command(
    name = "what-the-ip",
    about = "Fetch IP address information.",
    long_about = None,
    version
)]
struct Args {
    /// IP address to look up (defaults to your own IP)
    #[arg(short, long, value_name = "IP")]
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

    let client = IPInfoClient::new(args.proxy.as_deref(), args.timeout)?;
    let ip_info = client.fetch(args.ip.as_deref()).await?;
    ip_info.print(args.extra_metadata, args.json);

    Ok(())
}
