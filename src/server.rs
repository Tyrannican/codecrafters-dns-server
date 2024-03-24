use crate::message::{
    answer::DnsAnswer,
    header::{DnsHeader, DnsHeaderFlag},
    question::DnsQuestion,
    DnsMessage, IntoBytes,
};
use anyhow::Result;
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

    fn response_header(&self, recv_header: &DnsHeader) -> DnsHeader {
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

    fn forward(
        &self,
        resolver_addr: &String,
        src_buf: &[u8],
    ) -> Result<(DnsHeader, Vec<DnsQuestion>, Vec<DnsAnswer>)> {
        anyhow::ensure!(resolver_addr.split_once(':').is_some());

        let mut dst_buf = [0; 512];
        let recv_header = DnsHeader::from_bytes(&src_buf[..12]);
        let (_, questions) = DnsQuestion::from_bytes(recv_header.question_records, &src_buf[12..])?;
        let mut answers = vec![];

        for question in questions.iter() {
            let mut forward_header = recv_header.clone();
            forward_header.question_records = 1;

            let message = DnsMessage {
                header: forward_header,
                question_records: vec![question.clone()],
                answer_records: vec![],
            };

            let message = message.into_bytes();
            self.connection.send_to(&message, resolver_addr)?;
            self.connection.recv_from(&mut dst_buf)?;
            let response_header = DnsHeader::from_bytes(&dst_buf[..12]);
            let (ans_offset, response_questions) =
                DnsQuestion::from_bytes(response_header.question_records, &dst_buf[12..])?;

            for rq in response_questions.into_iter() {
                answers.push(DnsAnswer::from_question(&rq, &dst_buf[12 + ans_offset..]));
            }

            println!("Answer buffer: {:?}", &dst_buf[12 + ans_offset..]);

            dst_buf.fill(0);
        }

        Ok((recv_header, questions, answers))
    }

    pub(crate) fn listen(&self, resolver: &Option<String>) -> Result<()> {
        let mut buffer = [0; 512];

        loop {
            match self.connection.recv_from(&mut buffer) {
                Ok((size, source)) => {
                    println!("Received {} bytes from {}", size, source);
                    let Some(resolver_addr) = resolver else {
                        anyhow::bail!("need a resolver");
                    };
                    let (header, questions, answers) = self.forward(resolver_addr, &buffer)?;
                    //let header = DnsHeader::from_bytes(&buffer[..12]);
                    //let questions =
                    //    DnsQuestion::from_bytes(header.question_records, &buffer[12..])?;

                    //let mut answers = vec![];
                    //for question in questions.iter() {
                    //    answers.push(DnsAnswer::from_question(question));
                    //}

                    let response_header = self.response_header(&header);
                    let message = DnsMessage {
                        header: response_header,
                        question_records: questions,
                        answer_records: answers,
                    };

                    let response = message.into_bytes();
                    self.connection
                        .send_to(&response, source)
                        .expect("Failed to send response");
                }
                Err(e) => {
                    eprintln!("Error receiving data: {}", e);
                    anyhow::bail!("something went wrong")
                }
            }
        }
    }
}
