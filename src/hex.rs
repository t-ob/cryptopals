use crate::common::codec::DecodeError;

const ENCODE_MAPPING: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
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
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Err(DecodeError::InvalidCharacter),
    Ok(10),
    Ok(11),
    Ok(12),
    Ok(13),
    Ok(14),
    Ok(15),
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
    Ok(10),
    Ok(11),
    Ok(12),
    Ok(13),
    Ok(14),
    Ok(15),
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
];

pub fn encode(buffer: &[u8]) -> String {
    buffer
        .iter()
        .flat_map(|b| {
            [
                ENCODE_MAPPING[(b >> 4) as usize],
                ENCODE_MAPPING[(b & 0xf) as usize],
            ]
        })
        .collect()
}

fn decode_char(c: &char) -> Result<u8, DecodeError> {
    *DECODE_MAPPING
        .get(*c as usize)
        .ok_or(DecodeError::OutOfBounds)?
}

pub fn decode(string: &str) -> Result<Vec<u8>, DecodeError> {
    let mut result = vec![];

    let string_chars = string.chars().collect::<Vec<_>>();

    for pair in string_chars.chunks(2) {
        match pair {
            [h, l] => result.push(decode_char(h)? << 4 | decode_char(l)?),
            _ => return Err(DecodeError::InvalidLength),
        }
    }

    Ok(result)
}
