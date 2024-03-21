use crate::{
    message::IntoBytes,
    utils::{parse_domain_name, DnsRecordClass, DnsRecordType},
};

pub(crate) struct DnsQuestion {
    pub(crate) name: String,
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
            name: String::from(domain),
            record_type: record_type.to_value(),
            record_class: record_class.to_value(),
        }
    }
}

impl IntoBytes for DnsQuestion {
    fn into_bytes(self) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend(parse_domain_name(&self.name));
        buf.extend(self.record_type.to_be_bytes());
        buf.extend(self.record_class.to_be_bytes());

        buf
    }
}
