pub(crate) mod answer;
pub(crate) mod header;
pub(crate) mod question;

use answer::DnsAnswer;
use header::DnsHeader;
use question::DnsQuestion;

pub(crate) trait IntoBytes {
    fn into_bytes(self) -> Vec<u8>;
}

pub(crate) struct DnsMessage {
    pub(crate) header: DnsHeader,
    pub(crate) question_records: Vec<DnsQuestion>,
    pub(crate) answer_records: Vec<DnsAnswer>,
}

impl IntoBytes for DnsMessage {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(self.header.into_bytes());
        for q in self.question_records {
            buf.extend(q.into_bytes());
        }

        for a in self.answer_records {
            buf.extend(a.into_bytes());
        }

        buf
    }
}
