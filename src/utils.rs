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

pub(crate) fn get_bits(value: u16, bits: u8, offset: u8) -> u16 {
    let mask = (1 << bits) - 1;
    (value >> offset) & mask as u16
}
