pub fn xor_buffers(lhs: &[u8], rhs: &[u8]) -> Vec<u8> {
    lhs.iter().zip(rhs).map(|(x, y)| x ^ y).collect()
}
