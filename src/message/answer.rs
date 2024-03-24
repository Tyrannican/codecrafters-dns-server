use crate::message::{question::DnsQuestion, IntoBytes};
use crate::utils::*;

use std::net::Ipv4Addr;

pub(crate) struct DnsAnswer {
    pub(crate) domain: Vec<u8>,
    pub(crate) record_type: u16,
    pub(crate) record_class: u16,
    pub(crate) ttl: u32,
    pub(crate) length: u16,
    pub(crate) data: u32,
}

impl DnsAnswer {
    pub(crate) fn from_question(question: &DnsQuestion, buffer: &[u8]) -> Self {
        let record_t = DnsRecordType::from_value(question.record_type);

        // NOTE: + 8 accounts for the length of the remaining fields
        let data_section = &buffer[question.domain.len() + 8..];
        let (length, data) = parse_data(record_t, &data_section);

        Self {
            domain: question.domain.clone(),
            record_type: question.record_type,
            record_class: question.record_class,
            ttl: 60,
            length,
            data,
        }
    }
}

impl IntoBytes for DnsAnswer {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = vec![];

        buf.extend(self.domain);
        buf.extend(self.record_type.to_be_bytes());
        buf.extend(self.record_class.to_be_bytes());
        buf.extend(self.ttl.to_be_bytes());
        buf.extend(self.length.to_be_bytes());
        buf.extend(self.data.to_be_bytes());

        buf
    }
}

// TODO: Improve
fn parse_data(record_type: DnsRecordType, data: &[u8]) -> (u16, u32) {
    match record_type {
        DnsRecordType::A => {
            let length = u16::from_be_bytes([data[0], data[1]]);
            let ip = Ipv4Addr::new(data[2], data[3], data[4], data[5]);
            let octets = ip.octets();

            return (length, u32::from_be_bytes(octets));
        }
        _ => (0, 0),
    }
}
