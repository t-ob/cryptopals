use std::io;

use cryptopals::common::plaintext_util;
use cryptopals::hex;
use cryptopals::xor;

fn main() -> io::Result<()> {
    let mut ciphertexts = vec![];
    for line in io::stdin().lines() {
        ciphertexts.push(hex::decode(&line?)?)
    }

    let mut best_scores = vec![0; ciphertexts.len()];
    let mut best_candidates = ciphertexts
        .iter()
        .map(|bytes| vec![0; bytes.len()])
        .collect::<Vec<_>>();

    for (idx, ciphertext) in ciphertexts.iter().enumerate() {
        let mut scores = vec![0; 1 << 8];
        let mut plaintext_candidates = vec![vec![0; ciphertext.len()]; 1 << 8];
        for byte in 0u8..=u8::max_value() {
            let key_candidate = vec![byte; ciphertext.len()];

            let plaintext_candidate = xor::xor_buffers(ciphertext, &key_candidate);

            scores[byte as usize] = plaintext_util::english_score(&plaintext_candidate);
            plaintext_candidates[byte as usize].copy_from_slice(&plaintext_candidate);
        }

        let best_score_candidate = scores
            .iter()
            .zip(plaintext_candidates)
            .max_by_key(|(x, _)| *x)
            .unwrap();

        best_scores[idx] = *best_score_candidate.0;
        best_candidates[idx].copy_from_slice(&best_score_candidate.1);
    }

    let plaintext_bytes = best_scores
        .iter()
        .zip(best_candidates)
        .max_by_key(|(x, _)| *x)
        .unwrap()
        .1;

    println!("{}", String::from_utf8(plaintext_bytes).unwrap());

    Ok(())
}
