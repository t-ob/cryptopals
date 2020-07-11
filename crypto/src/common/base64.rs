const INDEX: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

const REVERSE_INDEX: [Option<u8>; 128] = [
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
    Some(62),
    None,
    None,
    None,
    Some(63),
    Some(52),
    Some(53),
    Some(54),
    Some(55),
    Some(56),
    Some(57),
    Some(58),
    Some(59),
    Some(60),
    Some(61),
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
    Some(10),
    Some(11),
    Some(12),
    Some(13),
    Some(14),
    Some(15),
    Some(16),
    Some(17),
    Some(18),
    Some(19),
    Some(20),
    Some(21),
    Some(22),
    Some(23),
    Some(24),
    Some(25),
    None,
    None,
    None,
    None,
    None,
    None,
    Some(26),
    Some(27),
    Some(28),
    Some(29),
    Some(30),
    Some(31),
    Some(32),
    Some(33),
    Some(34),
    Some(35),
    Some(36),
    Some(37),
    Some(38),
    Some(39),
    Some(40),
    Some(41),
    Some(42),
    Some(43),
    Some(44),
    Some(45),
    Some(46),
    Some(47),
    Some(48),
    Some(49),
    Some(50),
    Some(51),
    None,
    None,
    None,
    None,
    None,
];

const PADDING: char = '=';
const PADDING_U8: u8 = PADDING as u8;

pub fn encode(bytes: &[u8]) -> String {
    let capacity = 4 * ((bytes.len() + 2) / 3);

    let mut out = String::with_capacity(capacity);

    let mut chunks = bytes.chunks_exact(3);

    for chunk in &mut chunks {
        let b0 = chunk[0];
        let b1 = chunk[1];
        let b2 = chunk[2];
        let i0: usize = (b0 >> 2).into();
        let i1: usize = (((b0 & 0x3) << 4) | (b1 >> 4)).into();
        let i2: usize = (((b1 & 0xF) << 2) | (b2 >> 6)).into();
        let i3: usize = (b2 & 0x3F).into();

        out.push(INDEX[i0]);
        out.push(INDEX[i1]);
        out.push(INDEX[i2]);
        out.push(INDEX[i3]);
    }

    match chunks.remainder() {
        [b0] => {
            let i0: usize = (b0 >> 2).into();
            let i1: usize = ((b0 & 0x3) << 4).into();
            out.push(INDEX[i0]);
            out.push(INDEX[i1]);
            out.push(PADDING);
            out.push(PADDING);
        }
        [b0, b1] => {
            let i0: usize = (b0 >> 2).into();
            let i1: usize = (((b0 & 0x3) << 4) | (b1 >> 4)).into();
            let i2: usize = ((b1 & 0xF) << 2).into();
            out.push(INDEX[i0]);
            out.push(INDEX[i1]);
            out.push(INDEX[i2]);
            out.push(PADDING);
        }
        _ => (),
    }

    out
}

pub fn decode(string: &str) -> Result<Vec<u8>, Vec<u8>> {
    let capacity = 3 * ((string.len() + 3) / 4);

    let mut out: Vec<u8> = Vec::with_capacity(capacity);

    let chars = string.as_bytes();

    let mut chunks = chars.chunks_exact(4);

    for chunk in &mut chunks {
        match *chunk {
            [i0, i1, PADDING_U8, PADDING_U8] => {
                match [REVERSE_INDEX[i0 as usize], REVERSE_INDEX[i1 as usize]] {
                    [Some(b0), Some(b1)] => {
                        out.push((b0 << 2) | (b1 >> 4));
                    }
                    _ => return Err(Vec::from(chunk)),
                }
            }
            [i0, i1, i2, PADDING_U8] => {
                match [
                    REVERSE_INDEX[i0 as usize],
                    REVERSE_INDEX[i1 as usize],
                    REVERSE_INDEX[i2 as usize],
                ] {
                    [Some(b0), Some(b1), Some(b2)] => {
                        out.push((b0 << 2) | (b1 >> 4));
                        out.push((b1 << 4) | (b2 >> 2));
                    }
                    _ => return Err(Vec::from(chunk)),
                }
            }
            [i0, i1, i2, i3] => {
                match [
                    REVERSE_INDEX[i0 as usize],
                    REVERSE_INDEX[i1 as usize],
                    REVERSE_INDEX[i2 as usize],
                    REVERSE_INDEX[i3 as usize],
                ] {
                    [Some(b0), Some(b1), Some(b2), Some(b3)] => {
                        out.push((b0 << 2) | (b1 >> 4));
                        out.push((b1 << 4) | (b2 >> 2));
                        out.push((b2 << 6) | b3);
                    }
                    _ => return Err(Vec::from(chunk)),
                }
            }
            _ => (),
        }
    }

    let remainder = chunks.remainder();

    if !remainder.is_empty() {
        return Err(Vec::from(remainder));
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod encode {
        use super::*;
        #[test]
        fn test_zero_bytes_maps_to_empty_string() {
            assert_eq!(encode(&[]), String::from(""))
        }
        #[test]
        fn test_three_bytes_maps_to_four_b64_chars() {
            assert_eq!(encode(&[0x49, 0x27, 0x6d]), String::from("SSdt"))
        }
        #[test]
        fn test_one_byte_maps_to_four_b64_chars_with_padding() {
            assert_eq!(encode(&[0x49]), String::from("SQ=="))
        }
        #[test]
        fn test_two_bytes_maps_to_four_b64_chars_with_padding() {
            assert_eq!(encode(&[0x49, 0x27]), String::from("SSc="))
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn test_decoding_chars_with_total_not_divisible_by_four_errors() {
            assert_eq!(decode("SSd"), Err(Vec::from([0x53, 0x53, 0x64])));

            assert_eq!(decode("SS"), Err(Vec::from([0x53, 0x53])));

            assert_eq!(decode("S"), Err(Vec::from([0x53])))
        }

        #[test]
        fn test_decoding_four_chars_maps_to_three_bytes() {
            assert_eq!(decode("SSdt").unwrap(), Vec::from([0x49, 0x27, 0x6d]))
        }

        #[test]
        fn test_decoding_with_one_padding_maps_to_two_bytes() {
            assert_eq!(decode("SSc=").unwrap(), Vec::from([0x49, 0x27]))
        }

        #[test]
        fn test_decoding_with_two_padding_maps_to_one_byte() {
            assert_eq!(decode("SQ==").unwrap(), Vec::from([0x49]))
        }
    }

    mod encode_decode {
        use super::*;

        #[test]
        #[ignore]
        fn test_encode_decode_over_all_3_byte_values() {
            for value in 0..0x1000000 {
                let bytes: [u8; 3] = [
                    ((value & 0xFF0000) >> 0x10) as u8,
                    ((value & 0xFF00) >> 0x8) as u8,
                    (value & 0xFF) as u8,
                ];
                let round_trip = decode(&encode(&bytes[..])).unwrap();
                let result: [u8; 3] = [round_trip[0], round_trip[1], round_trip[2]];

                assert_eq!(bytes, result);
            }
        }
    }
}
