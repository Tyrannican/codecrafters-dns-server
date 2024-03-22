use crate::{
    message::IntoBytes,
    utils::{parse_domain_name, DnsRecordClass, DnsRecordType},
};
use anyhow::{bail, Result};

pub(crate) struct DnsQuestion {
    pub(crate) name: Vec<u8>,
    pub(crate) record_type: u16,
    pub(crate) record_class: u16,
}

impl DnsQuestion {
    pub(crate) fn new(
        domain: &str,
        record_type: DnsRecordType,
        record_class: DnsRecordClass,
    ) -> Self {
        Self {
            name: parse_domain_name(domain),
            record_type: record_type.to_value(),
            record_class: record_class.to_value(),
        }
    }

    // TODO: Improve cause this is awful
    pub(crate) fn from_bytes(total: u16, mut buffer: &[u8]) -> Result<Vec<Self>> {
        let mut questions = vec![];
        for _ in 0..total {
            let Some(idx) = buffer.iter().position(|&b| b == 0) else {
                bail!("there should be at least a null byte here");
            };

            let domain = &buffer[..=idx];
            let record_type = u16::from_be_bytes([buffer[idx + 1], buffer[idx + 2]]);
            let record_class = u16::from_be_bytes([buffer[idx + 3], buffer[idx + 4]]);
            questions.push(DnsQuestion {
                name: domain.to_vec(),
                record_type,
                record_class,
            });

            buffer = &buffer[idx + 5..];
        }

        Ok(questions)
    }
}

impl IntoBytes for DnsQuestion {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(self.name);
        buf.extend(self.record_type.to_be_bytes());
        buf.extend(self.record_class.to_be_bytes());

        buf
    }
}
