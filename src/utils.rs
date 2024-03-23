// NOTE: Some record types from https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2
// have been omitted as they are either experimental or obsolete
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
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

impl DnsRecordType {
    pub(crate) fn to_value(&self) -> u16 {
        match *self {
            Self::A => 1,
            Self::NS => 2,
            Self::CNAME => 5,
            Self::SOA => 6,
            Self::WKS => 11,
            Self::PTR => 12,
            Self::HINFO => 13,
            Self::MINFO => 14,
            Self::MX => 15,
            Self::TXT => 16,
        }
    }

    pub(crate) fn from_value(value: u16) -> Self {
        match value {
            1 => Self::A,
            2 => Self::NS,
            5 => Self::CNAME,
            6 => Self::SOA,
            11 => Self::WKS,
            12 => Self::PTR,
            13 => Self::HINFO,
            14 => Self::MINFO,
            15 => Self::MX,
            16 => Self::TXT,
            _ => unimplemented!("no others"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum DnsRecordClass {
    IN,
    CS,
    CH,
    HS,
}

impl DnsRecordClass {
    pub(crate) fn to_value(&self) -> u16 {
        match *self {
            Self::IN => 1,
            Self::CS => 2,
            Self::CH => 3,
            Self::HS => 4,
        }
    }
}

pub(crate) fn parse_domain_name(domain: &str) -> Vec<u8> {
    let mut encoded = vec![];
    for label in domain.split('.') {
        encoded.push(label.len() as u8);
        encoded.extend(label.as_bytes());
    }
    encoded.push(0);

    encoded
}
