use crate::net::dns::{
    DNSFlagsHeader, DNSHeaders, DNSMessage, DNSQuestion, OpCode, RecordClass, RecordType,
    ResponseCode,
};
use crate::util::bytes_to_hex;

use std::net::{SocketAddr, UdpSocket};

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

pub fn resolve(domain_name: &str) -> Result<String, std::io::Error> {
    let bytes = build_resolve_bytes(domain_name);
    let socket = find_local_udp_socket()?;
    let mut response = [0u8; 512];
    socket.send_to(&bytes, "8.8.8.8:53")?;
    socket.recv(&mut response)?;
    let result = bytes_to_hex(&response);
    println!("Result {}", result);
    Ok(result)
}

pub fn build_resolve_bytes(domain_name: &str) -> Vec<u8> {
    let message = DNSMessage {
        headers: DNSHeaders {
            transaction_id: 56130,
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
        sections: vec![DNSQuestion {
            domain_name,
            record_type: RecordType::A,
            class: RecordClass::Internet,
        }],
    };

    let mut header_buf = [0u8; (
        6 * 16
        // Header bits
    ) / 8];

    let mut question_suffix_buf = [0u8; (
        2 * 16
        // Rest of question
    ) / 8];

    let transaction_bytes = message.headers.transaction_id.to_be_bytes();
    header_buf[0] = transaction_bytes[0];
    header_buf[1] = transaction_bytes[1];
    let flags = message.headers.flags;
    header_buf[2] = pack_flags_byte_1(
        flags.is_reply,
        flags.op_code.value(),
        flags.is_authoritative_answer,
        flags.is_truncated,
        flags.recursion_desired,
    );
    header_buf[3] = pack_flags_byte_2(flags.recursion_available, flags.response_code.value());
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

    let question_section = &message.sections[0];

    // a domain name represented as a sequence of labels, where
    // each label consists of a length octet followed by that
    // number of octets.  The domain name terminates with the
    // zero length octet for the null label of the root.  Note
    // that this field may be an odd number of octets; no
    // padding is used.

    let domain_name = question_section.domain_name;
    let split_domain_name: Vec<&str> = domain_name.split('.').collect();
    let query_name_length = domain_name.len() + 1 /* "." becomes length + 1 */ + 1;
    let mut question_domain_name = vec![0u8; query_name_length];
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

    assert_eq!(question_section.record_type.value(), 1);
    let record_type_bytes = question_section.record_type.value().to_be_bytes();
    question_suffix_buf[0] = record_type_bytes[0];
    question_suffix_buf[1] = record_type_bytes[1];
    let class_bytes = question_section.class.value().to_be_bytes();
    question_suffix_buf[2] = class_bytes[0];
    question_suffix_buf[3] = class_bytes[1];

    let mut ret = header_buf.to_vec();
    ret.extend(question_domain_name);
    ret.extend_from_slice(&question_suffix_buf);

    ret
}

fn pack_flags_byte_1(qr: bool, opcode: u8, aa: bool, tc: bool, rd: bool) -> u8 {
    let mut ret = 0u8;
    ret |= (qr as u8) << 7;
    ret |= opcode << 3;
    ret |= (aa as u8) << 2;
    ret |= (tc as u8) << 1;
    ret |= rd as u8;
    ret
}

fn pack_flags_byte_2(ra: bool, rcode: u8) -> u8 {
    let mut ret = 0u8;
    ret |= (ra as u8) << 7;
    // Middle 4 bits are 0 and rcode should have top-4 bits empty
    ret |= rcode;
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::string_to_bytes;

    #[test]
    fn bit_shift() {
        let one = 1u8;
        assert_eq!(1, one);
        assert_eq!(2, one << 1);
        assert_eq!(4, one << 2);
        assert_eq!(128, one << 7);
    }

    /// [Source]https://www2.cs.duke.edu/courses/fall16/compsci356/DNS/DNS-primer.pdf)
    #[test]
    fn build_resolve_bytes_northeastern() {
        let result = build_resolve_bytes("www.northeastern.edu");
        assert_eq!(
            result,
            string_to_bytes(
                "
                db42 0100 0001 0000 0000 0000 0377 7777
                0c6e 6f72 7468 6561 7374 6572 6e03 6564
                7500 0001 0001
            "
            )
            .expect("")
        );
    }
}
