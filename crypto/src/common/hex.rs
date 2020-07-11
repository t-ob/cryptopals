const DECODE_INDEX: [Option<u8>; 128] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(0),
    Some(1),
    Some(2),
    Some(3),
    Some(4),
    Some(5),
    Some(6),
    Some(7),
    Some(8),
    Some(9),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(10),
    Some(11),
    Some(12),
    Some(13),
    Some(14),
    Some(15),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(10),
    Some(11),
    Some(12),
    Some(13),
    Some(14),
    Some(15),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
];

const ENCODE_INDEX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

pub fn decode(hex_string: &str) -> Result<Vec<u8>, Vec<u8>> {
    let mut out: Vec<u8> = Vec::with_capacity(hex_string.len() / 2);
    let mut chunks = hex_string.as_bytes().chunks_exact(2);
    for chunk in &mut chunks {
        if chunk[0] & 0x7F != chunk[0] || chunk[1] & 0x7F != chunk[1] {
            return Err(Vec::from(chunk));
        }
        let m_s_bits = DECODE_INDEX[(chunk[0] & 0x7F) as usize];
        let l_s_bits = DECODE_INDEX[(chunk[1] & 0x7F) as usize];

        match (m_s_bits, l_s_bits) {
            (Some(m_s_bits), Some(l_s_bits)) => out.push((m_s_bits << 4) | l_s_bits),
            _ => return Err(Vec::from(chunk)),
        }
    }

    let remainder = chunks.remainder();
    if !remainder.is_empty() {
        return Err(Vec::from(remainder));
    }

    Ok(out)
}

pub fn encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let m_s_bits = byte >> 4;
        let l_s_bits = byte & 0xF;
        out.push(ENCODE_INDEX[m_s_bits as usize]);
        out.push(ENCODE_INDEX[l_s_bits as usize]);
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    mod encode {
        use super::*;

        #[test]
        fn test_encode() {
            assert_eq!(&encode(&[0xDE, 0xAD, 0xBE, 0xEF])[..], "DEADBEEF")
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn test_decode() {
            assert_eq!(decode("DEADBEEF").unwrap()[..], [0xDE, 0xAD, 0xBE, 0xEF])
        }

        #[test]
        fn test_decode_returns_error_on_invalid_input() {
            assert_eq!(decode("DEADBEEG"), Err(Vec::from(['E' as u8, 'G' as u8])))
        }

        #[test]
        fn test_decode_returns_error_on_odd_length_input() {
            assert_eq!(decode("DEADBEE"), Err(Vec::from(['E' as u8])))
        }
    }
}
