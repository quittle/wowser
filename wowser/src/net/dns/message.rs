//! Structured defined in https://www.ietf.org/rfc/rfc1035.txt

use std::{convert::TryFrom, net::Ipv4Addr};

#[derive(Debug)]
pub struct DNSMessage {
    pub headers: DNSHeaders,
    pub sections: Vec<DNSRecord>,
}

#[derive(Debug)]
pub enum DNSRecord {
    Question(DNSQuestion),
    Answer(DNSAnswer),
}

#[derive(Debug)]
pub struct DNSHeaders {
    pub transaction_id: u16,
    pub flags: DNSFlagsHeader,
    pub num_of_questions: u16,
    pub num_of_answers: u16,
    pub num_of_authority_resource_records: u16,
    pub num_of_additional_records: u16,
}

#[derive(Debug)]
pub struct DNSFlagsHeader {
    /// otherwise query
    pub is_reply: bool,
    pub op_code: OpCode,
    pub is_authoritative_answer: bool,
    pub is_truncated: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub response_code: ResponseCode,
}

#[derive(Debug)]
pub enum OpCode {
    Query,
    InverseQuery,
    Status,
}

impl TryFrom<u8> for OpCode {
    type Error = &'static str;

    fn try_from(u4: u8) -> Result<OpCode, &'static str> {
        match u4 {
            0 => Ok(Self::Query),
            1 => Ok(Self::InverseQuery),
            2 => Ok(Self::Status),
            _ => Err("Unsupported opcode"),
        }
    }
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        match self {
            Self::Query => 0,
            Self::InverseQuery => 1,
            Self::Status => 2,
        }
    }
}

#[derive(Debug)]
pub enum ResponseCode {
    NoError,
    FormatError,
    ServerError,
    NameError,
    NotImplemented,
    Refused,
}

impl Into<u8> for ResponseCode {
    fn into(self) -> u8 {
        match self {
            Self::NoError => 0,
            Self::FormatError => 1,
            Self::ServerError => 2,
            Self::NameError => 3,
            Self::NotImplemented => 4,
            Self::Refused => 5,
        }
    }
}

impl TryFrom<u8> for ResponseCode {
    type Error = &'static str;

    fn try_from(u4: u8) -> Result<ResponseCode, &'static str> {
        match u4 {
            0 => Ok(Self::NoError),
            1 => Ok(Self::FormatError),
            2 => Ok(Self::ServerError),
            3 => Ok(Self::NameError),
            4 => Ok(Self::NotImplemented),
            5 => Ok(Self::Refused),
            _ => Err("Unsupported response code"),
        }
    }
}

#[derive(Debug)]
pub enum RecordClass {
    Internet,
    CSNET,
    Chaos,
    Hesiod,
}

impl AsRef<u16> for RecordClass {
    fn as_ref(&self) -> &u16 {
        match self {
            Self::Internet => &1,
            Self::CSNET => &2,
            Self::Chaos => &3,
            Self::Hesiod => &4,
        }
    }
}

impl Into<u16> for RecordClass {
    fn into(self) -> u16 {
        *(&self).as_ref()
    }
}

impl TryFrom<u16> for RecordClass {
    type Error = &'static str;

    fn try_from(literal: u16) -> Result<RecordClass, &'static str> {
        match literal {
            1 => Ok(Self::Internet),
            2 => Ok(Self::CSNET),
            3 => Ok(Self::Chaos),
            4 => Ok(Self::Hesiod),
            _ => Err("Unsupported record class"),
        }
    }
}

#[derive(Debug)]
pub enum RecordType {
    A,
    NameServer,
    MailDestination,
    MailForwarder,
    CanonicalName,
    StartOfZoneOfAuthority,
    MailboxDomainName,
    MailGroupMember,
    MailRenameDomainName,
    Null,
    WellKnownServiceDescription,
    Pointer,
    HostInfo,
    MailboxInfo,
    MailExchange,
    Text,
}

impl AsRef<u16> for RecordType {
    fn as_ref(&self) -> &u16 {
        match self {
            Self::A => &1,
            Self::NameServer => &2,
            Self::MailDestination => &3,
            Self::MailForwarder => &4,
            Self::CanonicalName => &5,
            Self::StartOfZoneOfAuthority => &6,
            Self::MailboxDomainName => &7,
            Self::MailGroupMember => &8,
            Self::MailRenameDomainName => &9,
            Self::Null => &10,
            Self::WellKnownServiceDescription => &11,
            Self::Pointer => &12,
            Self::HostInfo => &13,
            Self::MailboxInfo => &14,
            Self::MailExchange => &15,
            Self::Text => &16,
        }
    }
}

impl Into<u16> for RecordType {
    fn into(self) -> u16 {
        *(&self).as_ref()
    }
}

impl TryFrom<u16> for RecordType {
    type Error = &'static str;

    fn try_from(literal: u16) -> Result<RecordType, &'static str> {
        match literal {
            1 => Ok(Self::A),
            2 => Ok(Self::NameServer),
            3 => Ok(Self::MailDestination),
            4 => Ok(Self::MailForwarder),
            5 => Ok(Self::CanonicalName),
            6 => Ok(Self::StartOfZoneOfAuthority),
            7 => Ok(Self::MailboxDomainName),
            8 => Ok(Self::MailGroupMember),
            9 => Ok(Self::MailRenameDomainName),
            10 => Ok(Self::Null),
            11 => Ok(Self::WellKnownServiceDescription),
            12 => Ok(Self::Pointer),
            13 => Ok(Self::HostInfo),
            14 => Ok(Self::MailboxInfo),
            15 => Ok(Self::MailExchange),
            16 => Ok(Self::Text),
            _ => Err("Unsupported record type"),
        }
    }
}

#[derive(Debug)]
pub struct DNSQuestion {
    pub domain_name: String,
    pub record_type: RecordType,
    pub class: RecordClass,
}

#[derive(Debug)]
pub struct DNSAnswer {
    pub domain_name: String,
    pub record_type: RecordType,
    pub class: RecordClass,
    pub ttl: i32,
    pub rdata: RecordData,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Protocol {
    Unsuported,
    TCP,
    UDP,
    SMTP,
}

impl Protocol {
    /// Full list of values known by the machine comes from /etc/protocols
    #[allow(dead_code)]
    fn value(&self) -> u8 {
        match self {
            Self::Unsuported => 0,
            Self::TCP => 6,
            Self::UDP => 17,
            Self::SMTP => 25,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum RecordData {
    CanonicalName(String),
    HostInfo {
        cpu: String,
        os: String,
    },
    Mailbox(String),
    MailboxDelivery(String),
    MailboxForward(String),
    MailboxGroup(String),
    MailboxInfo {
        /// The domain name responsible for itself
        respsonsible_domain_name: String,
        /// The domain to send errors
        error_domain_name: String,
    },
    MailboxRename(String),
    MailboxExchange {
        /// Also called priority
        preference: u16,
        exchange_domain_name: String,
    },
    Null(String),
    NameServer(String),
    Pointer(String),
    StartOfZoneOfAuthority {
        master_domain_name: String,
        responsible_mailbox_domain_name: String,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum_ttl: u32,
    },
    Text(Vec<String>),
    A(Ipv4Addr),
    WellKnownServiceDescription {
        address: Ipv4Addr,
        protocol: Protocol,
        bitmap: Vec<u8>,
    },
}
