pub struct DNSMessage<'a> {
    pub headers: DNSHeaders,
    pub sections: Vec<DNSQuestion<'a>>,
}

pub struct DNSHeaders {
    pub transaction_id: u16,
    pub flags: DNSFlagsHeader,
    pub num_of_questions: u16,
    pub num_of_answers: u16,
    pub num_of_authority_resource_records: u16,
    pub num_of_additional_records: u16,
}

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

pub enum OpCode {
    Query,
    InverseQuery,
    Status,
}

impl OpCode {
    pub fn value(&self) -> u8 {
        match self {
            Self::Query => 0,
            Self::InverseQuery => 1,
            Self::Status => 2,
        }
    }
}

pub enum ResponseCode {
    NoError,
    FormatError,
    ServerError,
    NameError,
    NotImplemented,
    Refused,
}

impl ResponseCode {
    pub fn value(&self) -> u8 {
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

pub enum RecordClass {
    Internet,
    CSNET,
    Chaos,
    Hesiod,
}

impl RecordClass {
    pub fn value(&self) -> u16 {
        match self {
            Self::Internet => 1,
            Self::CSNET => 2,
            Self::Chaos => 3,
            Self::Hesiod => 4,
        }
    }
}

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

impl RecordType {
    pub fn value(&self) -> u16 {
        match self {
            Self::A => 1,
            Self::NameServer => 2,
            Self::MailDestination => 3,
            Self::MailForwarder => 4,
            Self::CanonicalName => 5,
            Self::StartOfZoneOfAuthority => 6,
            Self::MailboxDomainName => 7,
            Self::MailGroupMember => 8,
            Self::MailRenameDomainName => 9,
            Self::Null => 10,
            Self::WellKnownServiceDescription => 11,
            Self::Pointer => 12,
            Self::HostInfo => 13,
            Self::MailboxInfo => 14,
            Self::MailExchange => 15,
            Self::Text => 16,
        }
    }
}

pub struct DNSQuestion<'a> {
    pub domain_name: &'a str,
    pub record_type: RecordType,
    pub class: RecordClass,
}
