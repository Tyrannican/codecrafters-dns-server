use crate::{
    message::IntoBytes,
    utils::{get_bits, parse_domain_name, DnsRecordClass, DnsRecordType},
};
use anyhow::Result;

pub(crate) struct DnsQuestion {
    pub(crate) domain: Vec<u8>,
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
            domain: parse_domain_name(domain),
            record_type: record_type.to_value(),
            record_class: record_class.to_value(),
        }
    }

    // TODO: Improve cause this is awful
    pub(crate) fn from_bytes(buffer: &[u8]) -> Result<Vec<Self>> {
        let mut questions: Vec<DnsQuestion> = vec![];
        let mut idx = 0;
        let mut offset;
        let mut domain_buf = vec![];
        loop {
            if idx >= buffer.len() {
                break;
            }

            let byte = buffer[idx];
            let mut check = get_bits(byte as u16, 2, 6);
            if check == 3 {
                check = check & 0b00 << 6;
                offset = check as usize;
                'inner: for inner in &buffer[offset..] {
                    if *inner != 0 {
                        domain_buf.push(*inner);
                        continue;
                    }
                    domain_buf.push(0);

                    let record_type = u16::from_be_bytes([buffer[idx + 1], buffer[idx + 2]]);
                    let record_class = u16::from_be_bytes([buffer[idx + 3], buffer[idx + 4]]);
                    questions.push(DnsQuestion {
                        domain: domain_buf.clone(),
                        record_type,
                        record_class,
                    });
                    idx += 5;
                    domain_buf.clear();
                    break 'inner;
                }
            } else {
                if byte != 0 {
                    domain_buf.push(byte);
                    idx += 1;
                    continue;
                }
                domain_buf.push(0);
                let record_type = u16::from_be_bytes([buffer[idx + 1], buffer[idx + 2]]);
                let record_class = u16::from_be_bytes([buffer[idx + 3], buffer[idx + 4]]);
                questions.push(DnsQuestion {
                    domain: domain_buf.clone(),
                    record_type,
                    record_class,
                });
                idx += 5;
                domain_buf.clear();
            }
        }

        //let mut questions = vec![];
        //for _ in 0..total {
        //    let Some(idx) = buffer.iter().position(|&b| b == 0) else {
        //        bail!("there should be at least a null byte here");
        //    };

        //    let domain = &buffer[..=idx];
        //    let record_type = u16::from_be_bytes([buffer[idx + 1], buffer[idx + 2]]);
        //    let record_class = u16::from_be_bytes([buffer[idx + 3], buffer[idx + 4]]);
        //    questions.push(DnsQuestion {
        //        name: domain.to_vec(),
        //        record_type,
        //        record_class,
        //    });

        //    buffer = &buffer[idx + 5..];
        //}

        Ok(questions)
    }
}

impl IntoBytes for DnsQuestion {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(self.domain);
        buf.extend(self.record_type.to_be_bytes());
        buf.extend(self.record_class.to_be_bytes());

        buf
    }
}
