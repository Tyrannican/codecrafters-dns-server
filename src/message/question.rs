use crate::message::IntoBytes;

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
        let record_type = match record_type {
            DnsRecordType::A => 1,
            DnsRecordType::NS => 2,
            DnsRecordType::CNAME => 5,
            DnsRecordType::SOA => 6,
            DnsRecordType::WKS => 11,
            DnsRecordType::PTR => 12,
            DnsRecordType::HINFO => 13,
            DnsRecordType::MINFO => 14,
            DnsRecordType::MX => 15,
            DnsRecordType::TXT => 16,
        };

        let record_class = match record_class {
            DnsRecordClass::IN => 1,
            DnsRecordClass::CS => 2,
            DnsRecordClass::CH => 3,
            DnsRecordClass::HS => 4,
        };

        Self {
            name: String::from(domain),
            record_type,
            record_class,
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

fn parse_domain_name(domain: &str) -> Vec<u8> {
    let mut encoded = vec![];
    for label in domain.split('.') {
        encoded.push(label.len() as u8);
        encoded.extend(label.as_bytes());
    }
    encoded.push(0);

    encoded
}

// NOTE: Some record types from https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2
// have been omitted as they are either experimental or obsolete
pub(crate) enum DnsRecordType {
    A,
    NS,
    CNAME,
    SOA,
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT,
}

pub(crate) enum DnsRecordClass {
    IN,
    CS,
    CH,
    HS,
}
