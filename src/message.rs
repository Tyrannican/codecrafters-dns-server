pub(crate) mod header;
pub(crate) mod question;

use header::DnsHeader;

pub(crate) struct DnsMessage {
    pub(crate) header: DnsHeader,
}
