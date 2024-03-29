use crate::message::IntoBytes;
use crate::utils::get_bits;

#[derive(Default, Clone, Debug)]
pub(crate) struct DnsHeader {
    pub(crate) id: u16,
    pub(crate) flags: u16,
    pub(crate) question_records: u16,
    pub(crate) answer_records: u16,
    pub(crate) authority_records: u16,
    pub(crate) additional_records: u16,
}

impl DnsHeader {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        let header_info = bytes
            .chunks(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect::<Vec<u16>>();

        Self {
            id: header_info[0],
            flags: header_info[1],
            question_records: header_info[2],
            answer_records: header_info[3],
            authority_records: header_info[4],
            additional_records: header_info[5],
        }
    }

    pub(crate) fn set_flag(&mut self, flag: DnsHeaderFlag) {
        match flag {
            DnsHeaderFlag::Response => self.flags |= 1 << 15,
            DnsHeaderFlag::OpCode(code) => self.flags |= code << 11,
            DnsHeaderFlag::AuthoritativeAnswer => self.flags |= 1 << 10,
            // NOTE: Added truncation here as it's needed for TCP
            // however not used in this implementation as its only UDP for now
            DnsHeaderFlag::Truncation => self.flags |= 1 << 9,
            DnsHeaderFlag::RecursionDesired => self.flags |= 1 << 8,
            DnsHeaderFlag::RecursionAvailable => self.flags |= 1 << 7,
            // NOTE: There is three reserved bits here which can be ignored
            DnsHeaderFlag::ResponseCode(code) => self.flags |= code << 0,
        }
    }

    pub(crate) fn get_flag(&self, flag: DnsHeaderFlag) -> u16 {
        match flag {
            DnsHeaderFlag::Response => get_bits(self.flags, 1, 15),
            DnsHeaderFlag::OpCode(_) => get_bits(self.flags, 4, 11),
            DnsHeaderFlag::AuthoritativeAnswer => get_bits(self.flags, 1, 10),
            // NOTE: Added truncation here as it's needed for TCP
            // however not used in this implementation as its only UDP for now
            DnsHeaderFlag::Truncation => get_bits(self.flags, 1, 9),
            DnsHeaderFlag::RecursionDesired => get_bits(self.flags, 1, 8),
            DnsHeaderFlag::RecursionAvailable => get_bits(self.flags, 1, 7),
            // NOTE: There is three reserved bits here which can be ignored
            DnsHeaderFlag::ResponseCode(_) => get_bits(self.flags, 4, 0),
        }
    }
}

impl IntoBytes for DnsHeader {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(self.id.to_be_bytes());
        buf.extend(self.flags.to_be_bytes());
        buf.extend(self.question_records.to_be_bytes());
        buf.extend(self.answer_records.to_be_bytes());
        buf.extend(self.authority_records.to_be_bytes());
        buf.extend(self.additional_records.to_be_bytes());

        buf
    }
}

#[allow(dead_code)]
pub(crate) enum DnsHeaderFlag {
    Response,
    OpCode(u16),
    AuthoritativeAnswer,
    Truncation,
    RecursionDesired,
    RecursionAvailable,
    ResponseCode(u16),
}
