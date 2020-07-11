pub fn xor_buffers(x: &[u8], y: &[u8]) -> Vec<u8> {
    x.iter()
        .zip(y.iter())
        .map(|(x, y)| *x ^ *y)
        .collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
