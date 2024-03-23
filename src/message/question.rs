use std::io::BufRead;

use crate::{message::IntoBytes, utils::get_bits};
use anyhow::Result;

pub(crate) struct DnsQuestion {
    pub(crate) domain: Vec<u8>,
    pub(crate) record_type: u16,
    pub(crate) record_class: u16,
}

impl DnsQuestion {
    // TODO: Improve cause this is awful
    pub(crate) fn from_bytes(total_q: u16, buffer: &[u8]) -> Result<Vec<Self>> {
        // NOTE: This might include the header so be aware
        let mut idx = 0;
        let mut parsed_q = 0;
        let mut domain_buf = vec![];
        let mut questions = vec![];

        loop {
            if idx >= buffer.len() || parsed_q == total_q {
                break;
            }

            let byte = buffer[idx];
            if is_compressed_domain(byte) {
                let (read, domain) = decompress_domain(byte, buffer)?;
                domain_buf.extend(domain);
                let tmp_idx = idx + read;
                questions.push(DnsQuestion {
                    domain: domain_buf.clone(),
                    record_type: u16::from_be_bytes([buffer[tmp_idx + 1], buffer[tmp_idx + 2]]),
                    record_class: u16::from_be_bytes([buffer[tmp_idx + 3], buffer[tmp_idx + 4]]),
                });
                idx += 5;
            } else {
                if byte != 0 {
                    domain_buf.push(byte);
                    idx += 1;
                    continue;
                }

                domain_buf.push(0);
                questions.push(DnsQuestion {
                    domain: domain_buf.clone(),
                    record_type: u16::from_be_bytes([buffer[idx + 1], buffer[idx + 2]]),
                    record_class: u16::from_be_bytes([buffer[idx + 3], buffer[idx + 4]]),
                });
                parsed_q += 1;
                idx += 5;
            }
        }

        Ok(questions)
    }
}

fn is_compressed_domain(byte: u8) -> bool {
    get_bits(byte as u16, 2, 6) == 3
}

fn decompress_domain(offset: u8, buffer: &[u8]) -> Result<(usize, Vec<u8>)> {
    let offset_idx = 0b00 << 6 & offset;
    dbg!(offset_idx);
    let mut offset_buf = &buffer[offset_idx as usize..];
    let mut domain_buf = vec![];
    let read = offset_buf.read_until(0, &mut domain_buf)?;

    Ok((read, domain_buf))
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
