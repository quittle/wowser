use super::{
    DNSAnswer, DNSFlagsHeader, DNSHeaders, DNSMessage, DNSQuestion, DNSRecord, OpCode, RecordClass,
    RecordData, RecordType, ResponseCode,
};
use crate::net::NETWORK_BUFFER_SIZE;
use crate::util::{
    get_bit, offset_bit_merge, u4_from_u8, u8_arr_to_u16, u8_to_i32, u8_to_str, Bit, Hashable,
    U4BitOffset,
};

use std::collections::HashMap;
use std::convert::TryFrom;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::{borrow::Cow, str};

fn find_local_udp_socket() -> Result<UdpSocket, std::io::Error> {
    let mut err;

    match UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], 2000))) {
        Ok(socket) => return Ok(socket),
        Err(e) => err = e,
    };

    for port in (3000..4000).step_by(13) {
        match UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], port))) {
            Ok(socket) => return Ok(socket),
            Err(e) => err = e,
        };
    }
    Err(err)
}

fn parse_dns_response(message: &[u8; NETWORK_BUFFER_SIZE]) -> Result<DNSMessage, String> {
    let transaction_id = u8_arr_to_u16(message[0], message[1]);
    let flags_a = message[2];
    let flags_b = message[3];
    let flags = DNSFlagsHeader {
        is_reply: get_bit(flags_a, Bit::Zero),
        op_code: OpCode::try_from(u4_from_u8(flags_a, U4BitOffset::One))?,
        is_authoritative_answer: get_bit(flags_a, Bit::Five),
        is_truncated: get_bit(flags_a, Bit::Six),
        recursion_desired: get_bit(flags_a, Bit::Seven),
        recursion_available: get_bit(flags_b, Bit::Zero),
        response_code: ResponseCode::try_from(u4_from_u8(flags_b, U4BitOffset::Four))?,
    };

    if get_bit(flags_b, Bit::One) || get_bit(flags_b, Bit::Two) || get_bit(flags_b, Bit::Three) {
        return Err(
            "Invalid message. Non-zero bits found in reserved field of flag header".to_string(),
        );
    }

    let num_of_questions = u8_arr_to_u16(message[4], message[5]);
    let num_of_answers = u8_arr_to_u16(message[6], message[7]);
    let num_of_authority_resource_records = u8_arr_to_u16(message[8], message[9]);
    let num_of_additional_records = u8_arr_to_u16(message[10], message[11]);

    let headers = DNSHeaders {
        transaction_id,
        flags,
        num_of_questions,
        num_of_answers,
        num_of_authority_resource_records,
        num_of_additional_records,
    };

    let mut sections = vec![];

    // Tracks the index of bytes interpreted so far
    let mut offset = 12;
    for _ in 0..num_of_questions {
        let mut domain_name = vec![];
        // This could cause issues if the offset or offset+len is outside the range of the message
        while message[offset] != 0 {
            let len = message[offset] as usize;
            offset += 1;
            domain_name.push(&message[offset..(offset + len)]);
            offset += len;
        }
        // Last bit is a 0
        offset += 1;
        let domain_name = domain_name.join(&b'.');
        let domain_name = u8_to_str(&domain_name)?.to_string();
        let record_type =
            RecordType::try_from(u8_arr_to_u16(message[offset], message[offset + 1]))?;
        offset += 2;
        let class = RecordClass::try_from(u8_arr_to_u16(message[offset], message[offset + 1]))?;
        offset += 2;

        let question = DNSQuestion {
            domain_name,
            record_type,
            class,
        };
        sections.push(DNSRecord::Question(question));
    }

    for _ in 0..num_of_answers {
        let (domain_name, new_offset) = compute_domain(message, offset)?;
        offset = new_offset;
        let record_type =
            RecordType::try_from(u8_arr_to_u16(message[offset], message[offset + 1]))?;
        offset += 2;
        let class = RecordClass::try_from(u8_arr_to_u16(message[offset], message[offset + 1]))?;
        offset += 2;
        let ttl = u8_to_i32(
            message[offset],
            message[offset + 1],
            message[offset + 2],
            message[offset + 3],
        );
        offset += 4;
        let rdata_len = u8_arr_to_u16(message[offset], message[offset + 1]) as usize;
        offset += 2;
        let rdata_bytes: &[u8] = &message[offset..offset + rdata_len];
        offset += rdata_len;
        let rdata = parse_rdata(&record_type, rdata_bytes, message, offset)?;
        sections.push(DNSRecord::Answer(DNSAnswer {
            domain_name,
            record_type,
            class,
            ttl,
            rdata,
        }))
    }

    Ok(DNSMessage { headers, sections })
}

fn parse_rdata(
    record_type: &RecordType,
    bytes: &[u8],
    message: &[u8],
    offset: usize,
) -> Result<RecordData, String> {
    match record_type {
        RecordType::A => {
            let bytes_len = bytes.len();
            if bytes_len != 4 {
                Err(format!("Expected 4 bytes but got {bytes_len}"))
            } else {
                Ok(RecordData::A(Ipv4Addr::new(
                    bytes[0], bytes[1], bytes[2], bytes[3],
                )))
            }
        }
        RecordType::CanonicalName => {
            let (domain, _offset) = compute_domain(message, offset)?;
            Ok(RecordData::CanonicalName(domain))
        }
        _ => Err(format!("Unsupported record type {:?}", record_type)),
    }
}

// Is 0 -> Done
// is 11xxxxxx -> Pointer
// is 00xxxxxx -> Label
/// Returns the domain name + the new offset
pub fn compute_domain(message: &[u8], offset: usize) -> Result<(String, usize), String> {
    if message[offset] == 0 {
        return Ok((String::new(), offset + 1));
    }

    let first_byte = message[offset];
    let second_byte = message[offset + 1];
    let first_bit = get_bit(first_byte, Bit::Zero);
    let second_bit = get_bit(first_byte, Bit::One);

    if first_bit && second_bit {
        let pointer = offset_bit_merge(first_byte, Bit::Two, second_byte) as usize;
        Ok((compute_domain(message, pointer)?.0, offset + 2))
    } else if !first_bit && !second_bit {
        let len = first_byte as usize;
        let cur_value = u8_to_str(&message[offset + 1..(offset + 1 + len)])?;
        let rest = compute_domain(message, offset + 1 + len)?;
        let value = format!("{cur_value}.{}", &rest.0);

        Ok((value, rest.1))
    } else {
        Err("Unsupported domain label identifier".to_string())
    }
}

fn proper_domain_name(domain_name: &str) -> Cow<str> {
    if domain_name.ends_with('.') {
        Cow::Borrowed(domain_name)
    } else {
        Cow::Owned(format!("{domain_name}."))
    }
}

#[derive(Debug)]
enum DnsResult {
    CanonicalName(String),
    A(Ipv4Addr),
    NotFound,
}

pub fn resolve_domain_name_to_ip(domain_name: &str) -> Result<Ipv4Addr, String> {
    recurse_resolve_domain_name_to_ip(domain_name, &mut HashMap::new())
}

fn recurse_resolve_domain_name_to_ip(
    domain_name: &str,
    dns_cache: &mut HashMap<String, DnsResult>,
) -> Result<Ipv4Addr, String> {
    let bytes = build_resolve_bytes(domain_name, domain_name.hash_u16());
    let socket = find_local_udp_socket().map_err(|e| e.to_string())?;
    let mut response = [0_u8; NETWORK_BUFFER_SIZE];
    let bytes_sent = socket
        .send_to(&bytes, "8.8.8.8:53")
        .map_err(|e| e.to_string())?;
    debug_assert_eq!(bytes_sent, bytes.len());
    socket.recv(&mut response).map_err(|e| e.to_string())?;

    let response = parse_dns_response(&response)?;

    dns_cache.insert(domain_name.into(), DnsResult::NotFound);
    for section in &response.sections {
        if let DNSRecord::Answer(DNSAnswer {
            domain_name,
            rdata,
            class: RecordClass::Internet,
            ..
        }) = section
        {
            let dns_result = match rdata {
                RecordData::A(ip) => DnsResult::A(*ip),
                RecordData::CanonicalName(cname) => DnsResult::CanonicalName(cname.clone()),
                _ => continue,
            };
            dns_cache.insert(domain_name.clone(), dns_result);
        }
    }

    resolve_dns(domain_name, dns_cache)
}

fn resolve_dns(
    domain_name: &str,
    dns_cache: &mut HashMap<String, DnsResult>,
) -> Result<Ipv4Addr, String> {
    let cname: String;
    let proper_domain_name = proper_domain_name(domain_name);
    match dns_cache.get(&proper_domain_name.to_string()) {
        Some(DnsResult::A(ip)) => return Ok(*ip),
        Some(DnsResult::CanonicalName(canonical_name)) => cname = canonical_name.clone(),
        Some(DnsResult::NotFound) => {
            return recurse_resolve_domain_name_to_ip(domain_name, dns_cache)
        }
        None => {
            return Err(format!(
                "No valid record found for {domain_name}. Cache: {:?}",
                dns_cache
            ))
        }
    }
    resolve_dns(&cname, dns_cache)
}

pub fn build_resolve_bytes(domain_name: &str, transaction_id: u16) -> Vec<u8> {
    let message = DNSMessage {
        headers: DNSHeaders {
            transaction_id,
            flags: DNSFlagsHeader {
                is_reply: false,
                op_code: OpCode::Query,
                is_authoritative_answer: false,
                is_truncated: false,
                recursion_desired: true,
                recursion_available: false,
                response_code: ResponseCode::NoError,
            },
            num_of_questions: 1,
            num_of_answers: 0,
            num_of_authority_resource_records: 0,
            num_of_additional_records: 0,
        },
        sections: vec![DNSRecord::Question(DNSQuestion {
            domain_name: domain_name.to_string(),
            record_type: RecordType::A,
            class: RecordClass::Internet,
        })],
    };

    let mut header_buf = [0_u8;
        (
            6 * 16
            // Header bits
        ) / 8];

    let mut question_suffix_buf = [0_u8;
        (
            2 * 16
            // Rest of question
        ) / 8];

    let transaction_bytes = message.headers.transaction_id.to_be_bytes();
    header_buf[0] = transaction_bytes[0];
    header_buf[1] = transaction_bytes[1];
    let flags = message.headers.flags;
    header_buf[2] = pack_flags_byte_1(
        flags.is_reply,
        flags.op_code as u8,
        flags.is_authoritative_answer,
        flags.is_truncated,
        flags.recursion_desired,
    );
    header_buf[3] = pack_flags_byte_2(flags.recursion_available, flags.response_code as u8);
    let question_count_bytes = message.headers.num_of_questions.to_be_bytes();
    header_buf[4] = question_count_bytes[0];
    header_buf[5] = question_count_bytes[1];
    let answer_count_bytes = message.headers.num_of_answers.to_be_bytes();
    header_buf[6] = answer_count_bytes[0];
    header_buf[7] = answer_count_bytes[1];
    let authority_resource_records_count_bytes = message
        .headers
        .num_of_authority_resource_records
        .to_be_bytes();
    header_buf[8] = authority_resource_records_count_bytes[0];
    header_buf[9] = authority_resource_records_count_bytes[1];
    let additional_resource_records_count_bytes =
        message.headers.num_of_additional_records.to_be_bytes();
    header_buf[10] = additional_resource_records_count_bytes[0];
    header_buf[11] = additional_resource_records_count_bytes[1];

    let question_section = match &message.sections[0] {
        DNSRecord::Question(q) => q,
        _ => panic!("Invalid record type"),
    };

    // a domain name represented as a sequence of labels, where
    // each label consists of a length octet followed by that
    // number of octets.  The domain name terminates with the
    // zero length octet for the null label of the root.  Note
    // that this field may be an odd number of octets; no
    // padding is used.

    let domain_name = question_section.domain_name.as_str();
    let split_domain_name: Vec<&str> = domain_name.split('.').collect();
    let query_name_length = domain_name.len() + 1 /* "." becomes length + 1 */ + 1;
    let mut question_domain_name = vec![0_u8; query_name_length];
    let mut index = 0;
    for part in split_domain_name {
        question_domain_name[index] = part.len() as u8;
        index += 1;
        for byte in part.as_bytes() {
            question_domain_name[index] = *byte;
            index += 1;
        }
    }
    // Final byte should already by 0
    debug_assert_eq!(query_name_length, question_domain_name.len());
    debug_assert_eq!(index, question_domain_name.len() - 1);

    assert_eq!(question_section.record_type.as_ref(), &1);
    let record_type_bytes = question_section.record_type.as_ref().to_be_bytes();
    question_suffix_buf[0] = record_type_bytes[0];
    question_suffix_buf[1] = record_type_bytes[1];
    let class_bytes = question_section.class.as_ref().to_be_bytes();
    question_suffix_buf[2] = class_bytes[0];
    question_suffix_buf[3] = class_bytes[1];

    let mut ret = header_buf.to_vec();
    ret.extend(question_domain_name);
    ret.extend_from_slice(&question_suffix_buf);

    ret
}

fn pack_flags_byte_1(qr: bool, opcode: u8, aa: bool, tc: bool, rd: bool) -> u8 {
    let mut ret = 0_u8;
    ret |= (qr as u8) << 7;
    ret |= opcode << 3;
    ret |= (aa as u8) << 2;
    ret |= (tc as u8) << 1;
    ret |= rd as u8;
    ret
}

fn pack_flags_byte_2(ra: bool, rcode: u8) -> u8 {
    let mut ret = 0_u8;
    ret |= (ra as u8) << 7;
    // Middle 4 bits are 0 and rcode should have top-4 bits empty
    ret |= rcode;
    ret
}

#[cfg(test)]
mod tests {
    use super::super::super::NETWORK_BUFFER_SIZE;
    use super::*;
    use crate::util::string_to_bytes;

    #[test]
    fn bit_shift() {
        let one = 1_u8;
        assert_eq!(1, one);
        assert_eq!(2, one << 1);
        assert_eq!(4, one << 2);
        assert_eq!(128, one << 7);
    }

    /// [Source](https://www2.cs.duke.edu/courses/fall16/compsci356/DNS/DNS-primer.pdf)
    #[test]
    fn build_resolve_bytes_northeastern() {
        let result = build_resolve_bytes("www.northeastern.edu", 1234);
        assert_eq!(
            result,
            string_to_bytes(
                "
                04d2 0100 0001 0000 0000 0000 0377 7777
                0c6e 6f72 7468 6561 7374 6572 6e03 6564
                7500 0001 0001
            "
            )
            .expect("")
        );
    }

    #[test]
    fn test_parse_dns_response() {
        let bytes = string_to_bytes(
            "db42 8180 0001 0001 0000 0000 0377 7777
        0c6e 6f72 7468 6561 7374 6572 6e03 6564
        7500 0001 0001 c00c 0001 0001 0000 0258
        0004 9b21 1144
        ",
        )
        .expect("");

        let mut byte_arr = [0_u8; NETWORK_BUFFER_SIZE];
        byte_arr[..bytes.len()].copy_from_slice(bytes.as_ref());

        parse_dns_response(&byte_arr).expect("Must be valid");
    }

    #[test]
    fn test_dns_resolve() {
        let test_resolve = |domain| {
            let result = resolve_domain_name_to_ip(domain);
            assert!(
                result.is_ok(),
                "Failed to resolve {domain}: {}",
                result.unwrap_err()
            )
        };
        test_resolve("example.com");
        test_resolve("www.example.com");
        test_resolve("google.com");
        test_resolve("www.google.com");
        test_resolve("amazon.com");
        test_resolve("www.amazon.com");
        test_resolve("www.northeastern.edu");
        // api.amazonvideo.com has a complex chain of CNAMEs
        test_resolve("complex.api.amazonvideo.com");
    }
}
