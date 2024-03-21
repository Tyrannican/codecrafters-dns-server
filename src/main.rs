pub(crate) mod message;
pub(crate) mod server;
pub(crate) mod utils;

use server::DnsServer;

fn main() -> anyhow::Result<()> {
    println!("Logs from your program will appear here!");
    let dns_server = DnsServer::new("127.0.0.1:2053");
    dns_server.listen();

    Ok(())
}
