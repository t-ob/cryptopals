use std::cmp;

pub fn hamming_distance(lhs: &[u8], rhs: &[u8]) -> u32 {
    lhs.iter()
        .zip(rhs)
        .map(|(l, r)| (l ^ r).count_ones())
        .sum::<u32>()
        + 8 * (cmp::max(lhs.len(), rhs.len()) - cmp::min(lhs.len(), rhs.len())) as u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hamming_distance() {
        assert_eq!(
            37,
            super::hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes())
        )
    }
}
