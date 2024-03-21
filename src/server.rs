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
                    let recv_header = DnsHeader::from_bytes(&buffer[..12]);
                    // NOTE: 0 is passed here as it isn't used
                    let op_code = recv_header.get_flag(DnsHeaderFlag::OpCode(0));

                    let mut header = DnsHeader::new(recv_header.id);
                    header.set_flag(DnsHeaderFlag::Response);
                    header.set_flag(DnsHeaderFlag::OpCode(op_code));
                    if recv_header.get_flag(DnsHeaderFlag::RecursionDesired) > 0 {
                        header.set_flag(DnsHeaderFlag::RecursionDesired);
                    }

                    if op_code == 0 {
                        header.set_flag(DnsHeaderFlag::ResponseCode(0));
                    } else {
                        header.set_flag(DnsHeaderFlag::ResponseCode(4));
                    }

                    header.question_records = 1;
                    header.answer_records = 1;

                    let question =
                        DnsQuestion::new("codecrafters.io", DnsRecordType::A, DnsRecordClass::IN);

                    let answer =
                        DnsAnswer::new("codecrafters.io", DnsRecordType::A, DnsRecordClass::IN);

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
