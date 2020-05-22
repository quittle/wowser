mod dns_resolve;
mod message;

pub use message::{
    DNSAnswer, DNSFlagsHeader, DNSHeaders, DNSMessage, DNSQuestion, DNSRecord, OpCode, RecordClass,
    RecordData, RecordType, ResponseCode,
};

pub use dns_resolve::{build_resolve_bytes, resolve_domain_name_to_ip};
