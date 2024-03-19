pub(crate) struct DnsHeader {
    pub(crate) id: u16,
    pub(crate) flags: u16,
    pub(crate) question_records: u16,
    pub(crate) answer_records: u16,
    pub(crate) authority_records: u16,
    pub(crate) additional_records: u16,
}

impl DnsHeader {
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

    pub(crate) fn as_bytes(self) -> [u8; 12] {
        // TODO: Improve
        let mut buf: [u8; 12] = [0; 12];
        let mut t: Vec<u8> = vec![];
        t.extend(self.id.to_be_bytes());
        t.extend(self.flags.to_be_bytes());
        t.extend(self.question_records.to_be_bytes());
        t.extend(self.answer_records.to_be_bytes());
        t.extend(self.authority_records.to_be_bytes());
        t.extend(self.additional_records.to_be_bytes());

        buf.copy_from_slice(&t);

        buf
    }
}

pub(crate) enum DnsHeaderFlag {
    Response,
    OpCode(u16),
    AuthoritativeAnswer,
    Truncation,
    RecursionDesired,
    RecursionAvailable,
    ResponseCode(u16),
}
