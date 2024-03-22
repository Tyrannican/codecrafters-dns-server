use crate::message::IntoBytes;
use crate::utils::*;

use std::net::Ipv4Addr;

pub(crate) struct DnsAnswer {
    pub(crate) name: Vec<u8>,
    pub(crate) record_type: u16,
    pub(crate) record_class: u16,
    pub(crate) ttl: u32,
    pub(crate) length: u16,
    pub(crate) data: u32,
}

impl DnsAnswer {
    pub(crate) fn new(
        domain: &str,
        record_type: DnsRecordType,
        record_class: DnsRecordClass,
    ) -> Self {
        // TODO: Parse appropriately
        let (length, data) = parse_data(record_type);

        Self {
            name: parse_domain_name(domain),
            record_type: record_type.to_value(),
            record_class: record_class.to_value(),
            ttl: 60,
            length,
            data,
        }
    }
}

impl IntoBytes for DnsAnswer {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = vec![];

        buf.extend(self.name);
        buf.extend(self.record_type.to_be_bytes());
        buf.extend(self.record_class.to_be_bytes());
        buf.extend(self.ttl.to_be_bytes());
        buf.extend(self.length.to_be_bytes());
        buf.extend(self.data.to_be_bytes());

        buf
    }
}

// TODO: Improve
fn parse_data(record_type: DnsRecordType) -> (u16, u32) {
    match record_type {
        DnsRecordType::A => {
            let ip = Ipv4Addr::new(8, 8, 8, 8);
            let octets = ip.octets();

            return (octets.len() as u16, u32::from_be_bytes(octets));
        }
        _ => (0, 0),
    }
}
