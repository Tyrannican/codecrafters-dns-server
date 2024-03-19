use crate::message::header::{DnsHeader, DnsHeaderFlag};
use std::net::UdpSocket;

#[derive(Debug)]
pub(crate) struct DnsServer {
    connection: UdpSocket,
}

impl DnsServer {
    pub(crate) fn new(address: &str) -> Self {
        let connection = UdpSocket::bind(address).expect("Failed to bind to address");
        Self { connection }
    }

    pub(crate) fn listen(&self) {
        let mut buffer = [0; 512];

        loop {
            match self.connection.recv_from(&mut buffer) {
                Ok((size, source)) => {
                    println!("Received {} bytes from {}", size, source);

                    let mut header = DnsHeader {
                        id: 1234,
                        flags: 0,
                        additional_records: 0,
                        authority_records: 0,
                        question_records: 0,
                        answer_records: 0,
                    };
                    header.set_flag(DnsHeaderFlag::Response);
                    let response = header.as_bytes();
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
