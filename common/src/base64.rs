use std::cmp::max;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_bytes_maps_to_four_b64_chars() {
        assert_eq!(base64_encode(&[0x49, 0x27, 0x6d]), String::from("SSdt"))
    }

    #[test]
    fn test_one_byte_maps_to_four_b64_chars_with_padding() {
        assert_eq!(base64_encode(&[0x49]), String::from("SQ=="))
    }

    #[test]
    fn test_two_bytes_maps_to_four_b64_chars_with_padding() {
        assert_eq!(base64_encode(&[0x49, 0x27]), String::from("SSc="))
    }

    #[test]
    fn test_cryptopals_set_1_problem_1_passes() {
        let string = String::from("I'm killing your brain like a poisonous mushroom");
        assert_eq!(
            base64_encode(string.as_bytes()),
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
        )
    }
}

const INDEX: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];
const PADDING: char = '=';

pub fn base64_encode(bytes: &[u8]) -> String {
    let capacity = max(0, (bytes.len() - 1) / 4);

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
