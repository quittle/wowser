mod message;

pub use message::{
    DNSAnswer, DNSFlagsHeader, DNSHeaders, DNSMessage, DNSQuestion, DNSRecord, OpCode, RecordClass,
    RecordData, RecordType, ResponseCode,
};
