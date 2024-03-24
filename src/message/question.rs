use std::io::BufRead;

use crate::{message::IntoBytes, utils::get_bits};
use anyhow::Result;

#[derive(Debug, Clone)]
pub(crate) struct DnsQuestion {
    pub(crate) domain: Vec<u8>,
    pub(crate) record_type: u16,
    pub(crate) record_class: u16,
}

impl DnsQuestion {
    // TODO: Fix mad offset shenanigans
    pub(crate) fn from_bytes(total_q: u16, buffer: &[u8]) -> Result<(usize, Vec<Self>)> {
        let mut idx = 0;
        let mut domain_buf = vec![];
        let mut questions = vec![];

        loop {
            if idx >= buffer.len() || questions.len() == total_q as usize {
                break;
            }

            let byte = buffer[idx];
            if is_compressed_domain(byte) {
                let domain = decompress_domain(buffer[idx + 1], buffer)?;
                domain_buf.extend(domain);
                let tmp_idx = idx + 2;
                questions.push(DnsQuestion {
                    domain: domain_buf.clone(),
                    record_type: u16::from_be_bytes([buffer[tmp_idx], buffer[tmp_idx + 1]]),
                    record_class: u16::from_be_bytes([buffer[tmp_idx + 2], buffer[tmp_idx + 3]]),
                });
                idx += 4;
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
                idx += 5;
                domain_buf.clear();
            }
        }

        Ok((idx, questions))
    }
}

fn is_compressed_domain(byte: u8) -> bool {
    get_bits(byte as u16, 2, 6) == 3
}

fn decompress_domain(offset: u8, buffer: &[u8]) -> Result<Vec<u8>> {
    // NOTE: - 12 accounts for the header bytes
    let offset = offset - 12;
    let mut offset_buf = &buffer[offset as usize..];
    let mut domain_buf = vec![];
    offset_buf.read_until(0, &mut domain_buf)?;

    Ok(domain_buf)
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
