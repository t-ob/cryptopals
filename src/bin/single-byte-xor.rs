use std::io;

use cryptopals::hex;
use cryptopals::plaintext_util;
use cryptopals::xor;

fn main() -> io::Result<()> {
    let ciphertext_hex = io::stdin().lines().collect::<Result<String, _>>()?;
    let ciphertext = hex::decode(&ciphertext_hex)?;

    let mut scores = vec![0; 1 << 8];
    let mut plaintext_candidates = vec![vec![0; ciphertext.len()]; 1 << 8];
    for byte in 0u8..=u8::max_value() {
        let key_candidate = vec![byte; ciphertext.len()];

        let plaintext_candidate = xor::xor_buffers(&ciphertext, &key_candidate);

        scores[byte as usize] = plaintext_util::english_score(&plaintext_candidate);
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
