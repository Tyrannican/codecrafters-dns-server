use crate::message::{
    answer::DnsAnswer,
    header::{DnsHeader, DnsHeaderFlag},
    question::DnsQuestion,
    DnsMessage, IntoBytes,
};
use crate::utils::{DnsRecordClass, DnsRecordType};
use anyhow::{bail, Result};
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

    fn parse_header(&self, buffer: &[u8]) -> DnsHeader {
        let recv_header = DnsHeader::from_bytes(&buffer[..12]);
        // NOTE: 0 is passed here as it isn't used
        let op_code = recv_header.get_flag(DnsHeaderFlag::OpCode(0));

        let mut response_header = DnsHeader::new(recv_header.id);
        response_header.set_flag(DnsHeaderFlag::Response);
        response_header.set_flag(DnsHeaderFlag::OpCode(op_code));

        if recv_header.get_flag(DnsHeaderFlag::RecursionDesired) > 0 {
            response_header.set_flag(DnsHeaderFlag::RecursionDesired);
        }

        if op_code == 0 {
            response_header.set_flag(DnsHeaderFlag::ResponseCode(0));
        } else {
            response_header.set_flag(DnsHeaderFlag::ResponseCode(4));
        }

        response_header.question_records = recv_header.question_records;
        // NOTE: Is this going to be set to the number of questions in the recv_header?
        response_header.answer_records = recv_header.question_records;
        response_header.additional_records = recv_header.additional_records;
        response_header.authority_records = recv_header.authority_records;

        response_header
    }

    pub(crate) fn listen(&self) -> Result<()> {
        let mut buffer = [0; 512];

        loop {
            match self.connection.recv_from(&mut buffer) {
                Ok((size, source)) => {
                    println!("Received {} bytes from {}", size, source);
                    let header = self.parse_header(&buffer[..12]);

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
                    bail!("something went wrong")
                }
            }
        }
    }
}
