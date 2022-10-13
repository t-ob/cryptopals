use crate::common::codec::DecodeError;

const ENCODE_MAPPING: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

const DECODE_MAPPING: [Result<u8, DecodeError>; 128] = [
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Ok(62),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Ok(63),
    Ok(52),
    Ok(53),
    Ok(54),
    Ok(55),
    Ok(56),
    Ok(57),
    Ok(58),
    Ok(59),
    Ok(60),
    Ok(61),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Ok(0),
    Ok(1),
    Ok(2),
    Ok(3),
    Ok(4),
    Ok(5),
    Ok(6),
    Ok(7),
    Ok(8),
    Ok(9),
    Ok(10),
    Ok(11),
    Ok(12),
    Ok(13),
    Ok(14),
    Ok(15),
    Ok(16),
    Ok(17),
    Ok(18),
    Ok(19),
    Ok(20),
    Ok(21),
    Ok(22),
    Ok(23),
    Ok(24),
    Ok(25),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Ok(26),
    Ok(27),
    Ok(28),
    Ok(29),
    Ok(30),
    Ok(31),
    Ok(32),
    Ok(33),
    Ok(34),
    Ok(35),
    Ok(36),
    Ok(37),
    Ok(38),
    Ok(39),
    Ok(40),
    Ok(41),
    Ok(42),
    Ok(43),
    Ok(44),
    Ok(45),
    Ok(46),
    Ok(47),
    Ok(48),
    Ok(49),
    Ok(50),
    Ok(51),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
];

pub fn encode(buffer: &[u8]) -> String {
    buffer
        .chunks(3)
        .flat_map(|chunk| match chunk {
            [x] => [
                ENCODE_MAPPING[(x >> 2) as usize],
                ENCODE_MAPPING[((x & 3) << 4) as usize],
                '=',
                '=',
            ],
            [x, y] => [
                ENCODE_MAPPING[(x >> 2) as usize],
                ENCODE_MAPPING[(((x & 0x3) << 4) | ((y >> 4) & 0xF)) as usize],
                ENCODE_MAPPING[((y & 0xF) << 2) as usize],
                '=',
            ],
            [x, y, z] => [
                ENCODE_MAPPING[(x >> 2) as usize],
                ENCODE_MAPPING[(((x & 0x3) << 4) | ((y >> 4) & 0xF)) as usize],
                ENCODE_MAPPING[(((y & 0xF) << 2) | ((z >> 6) & 0x3)) as usize],
                ENCODE_MAPPING[(z & 0x3F) as usize],
            ],
            _ => unreachable!(),
        })
        .collect::<String>()
}

fn decode_char(c: &char) -> Result<u8, DecodeError> {
    *DECODE_MAPPING
        .get(*c as usize)
        .ok_or(DecodeError::OutOfBounds)?
}

pub fn decode(string: &str) -> Result<Vec<u8>, DecodeError> {
    let mut result = vec![];
    let string_chars = string.chars().collect::<Vec<_>>();

    for chunk in string_chars.chunks(4) {
        match chunk {
            [w, x, '=', '='] => {
                result.push((decode_char(w)? << 2) | (decode_char(x)? >> 4) & 0x3);
            }
            [w, x, y, '='] => {
                result.push((decode_char(w)? << 2) | (decode_char(x)? >> 4) & 0x3);
                result.push(((decode_char(x)? & 0xF) << 4) | (decode_char(y)? >> 2));
            }
            [w, x, y, z] => {
                result.push((decode_char(w)? << 2) | (decode_char(x)? >> 4) & 0x3);
                result.push(((decode_char(x)? & 0xF) << 4) | (decode_char(y)? >> 2));
                result.push(((decode_char(y)? & 0x3) << 6) | decode_char(z)?);
            }
            _ => return Err(DecodeError::InvalidLength),
        }
    }

    Ok(result)
}
