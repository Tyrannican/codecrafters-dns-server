use crate::message::IntoBytes;
use crate::utils::*;

pub(crate) struct DnsAnswer {}

impl IntoBytes for DnsAnswer {
    fn into_bytes(self) -> Vec<u8> {
        todo!()
    }
}
