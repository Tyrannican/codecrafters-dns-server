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
    pub(crate) question: DnsQuestion,
    pub(crate) answer: DnsAnswer,
}

impl IntoBytes for DnsMessage {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(self.header.into_bytes());
        buf.extend(self.question.into_bytes());
        buf.extend(self.answer.into_bytes());

        buf
    }
}
