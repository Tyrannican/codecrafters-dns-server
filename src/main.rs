pub(crate) mod message;
pub(crate) mod server;
pub(crate) mod utils;

use server::DnsServer;

fn main() -> anyhow::Result<()> {
    println!("Logs from your program will appear here!");
    //let dns_server = DnsServer::new("127.0.0.1:2053");
    //dns_server.listen()

    let mut buf: [u8; 512] = [12; 512];
    buf[21] = 0;
    let Some(idx) = &buf[12..].iter().position(|&b| b == 0) else {
        anyhow::bail!("need a null");
    };
    println!("The idx is {}", idx + 12);

    Ok(())
}
