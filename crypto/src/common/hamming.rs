pub fn distance(a: &[u8], b: &[u8]) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x ^ y).count_ones())
        .fold(0, |a, z| a + z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let a = "this is a test";
        let b = "wokka wokka!!!";

        assert_eq!(distance(a.as_bytes(), b.as_bytes()), 37)
    }
}
