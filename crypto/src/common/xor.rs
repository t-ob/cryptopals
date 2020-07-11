fn xor_buffers(x: &[u8], y: &[u8]) -> Vec<u8> {
    x.iter()
        .zip(y.iter())
        .map(|(x, y)| *x ^ *y)
        .collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::hex;

    #[test]
    fn test_xor_buffers_with_same_value_is_zero() {
        for x in 0..0x100 {
            assert_eq!(xor_buffers(&[x as u8], &[x as u8])[..], [0])
        }
    }

    #[test]
    fn test_xor_buffers_with_complement_is_u8_max() {
        for x in 0..0x100 {
            assert_eq!(xor_buffers(&[x as u8], &[!(x as u8)])[..], [0xFF])
        }
    }

    #[test]
    fn test_xor_buffers_returns_buffer_of_length_equal_to_smallest_length_of_inputs() {
        assert_eq!(xor_buffers(&[0, 1], &[0]), xor_buffers(&[0], &[0]));

        assert_eq!(xor_buffers(&[0], &[0, 1]), xor_buffers(&[0], &[0]))
    }

    #[test]
    fn test_cryptopals_set_1_problem_2_passes() {
        let x = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let y = hex::decode("686974207468652062756c6c277320657965").unwrap();
        let z = hex::decode("746865206b696420646f6e277420706c6179").unwrap();

        let result = xor_buffers(&x[..], &y[..]);

        assert_eq!(result, z)
    }
}
