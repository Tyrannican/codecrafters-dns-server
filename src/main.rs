pub(crate) mod message;
pub(crate) mod server;
pub(crate) mod utils;

use server::DnsServer;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// DNS resolver address in the form <addr>:<port>
    #[arg(long)]
    resolver: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    println!("Logs from your program will appear here!");
    let dns_server = DnsServer::new("127.0.0.1:2053");
    dns_server.listen(&cli.resolver)
}
