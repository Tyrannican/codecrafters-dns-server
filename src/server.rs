use crate::message::{
    answer::DnsAnswer,
    header::{DnsHeader, DnsHeaderFlag},
    question::DnsQuestion,
    DnsMessage, IntoBytes,
};
use crate::utils::{DnsRecordClass, DnsRecordType};
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

                    let mut header = DnsHeader::new(1234);
                    header.question_records = 1;
                    header.set_flag(DnsHeaderFlag::Response);

                    let question =
                        DnsQuestion::new("codecrafters.io", DnsRecordType::A, DnsRecordClass::IN);

                    let answer = DnsAnswer {};

                    let message = DnsMessage {
                        header,
                        question,
                        answer,
                    };

                    let response = message.into_bytes();
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
