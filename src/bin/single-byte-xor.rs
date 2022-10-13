use std::collections::HashMap;
use std::io;

use cryptopals::hex;
use cryptopals::xor;

fn score(buffer: &[u8]) -> u32 {
    let freqs = HashMap::from([
        (b' ', 15),
        (b'e', 13),
        (b'E', 13),
        (b't', 9),
        (b'T', 9),
        (b'a', 8),
        (b'A', 8),
        (b'o', 7),
        (b'O', 7),
        (b'i', 7),
        (b'I', 7),
        (b'n', 6),
        (b'N', 6),
        (b's', 6),
        (b'S', 6),
        (b'h', 6),
        (b'H', 6),
        (b'r', 6),
        (b'R', 6),
        (b'd', 4),
        (b'D', 4),
        (b'l', 4),
        (b'L', 4),
        (b'c', 2),
        (b'C', 2),
        (b'u', 2),
        (b'U', 2),
        (b'm', 2),
        (b'M', 2),
        (b'w', 2),
        (b'W', 2),
        (b'f', 2),
        (b'F', 2),
        (b'g', 2),
        (b'G', 2),
        (b'y', 2),
        (b'Y', 2),
        (b'p', 1),
        (b'P', 1),
        (b'b', 1),
        (b'B', 1),
    ]);

    buffer.iter().map(|b| freqs.get(b).unwrap_or(&0)).sum()
}

fn main() -> io::Result<()> {
    let ciphertext_hex = io::stdin().lines().collect::<Result<String, _>>()?;
    let ciphertext = hex::decode(&ciphertext_hex)?;

    let mut scores = vec![0; 1 << 8];
    let mut plaintext_candidates = vec![vec![0; ciphertext.len()]; 1 << 8];
    for byte in 0u8..=u8::max_value() {
        let key_candidate = vec![byte; ciphertext.len()];

        let plaintext_candidate = xor::xor_buffers(&ciphertext, &key_candidate);

        scores[byte as usize] = score(&plaintext_candidate);
        plaintext_candidates[byte as usize].copy_from_slice(&plaintext_candidate);
    }

    let plaintext_bytes = scores
        .iter()
        .zip(plaintext_candidates)
        .max_by_key(|(x, _)| *x)
        .unwrap()
        .1;

    println!("{}", String::from_utf8(plaintext_bytes).unwrap());

    Ok(())
}
