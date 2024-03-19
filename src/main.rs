use std::net::UdpSocket;

#[derive(Debug)]
struct DnsServer {
    connection: UdpSocket,
}

impl DnsServer {
    pub fn new(address: &str) -> Self {
        let connection = UdpSocket::bind(address).expect("Failed to bind to address");
        Self { connection }
    }

    pub fn listen(&self) {
        let mut buffer = [0; 512];

        loop {
            match self.connection.recv_from(&mut buffer) {
                Ok((size, source)) => {
                    println!("Received {} bytes from {}", size, source);
                    let response = [];
                    self.connection
                        .send_to(&response, source)
                        .expect("Failed to send response");
                }
                Err(e) => {
                    eprintln!("Error receiving data: {}", e);
                    break;
                }
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    println!("Logs from your program will appear here!");

    let dns_server = DnsServer::new("127.0.0.1:2053");
    dns_server.listen();

    Ok(())
}
