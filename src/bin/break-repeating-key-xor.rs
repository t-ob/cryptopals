use std::io;

use cryptopals::base64;
use cryptopals::common::bits;
use cryptopals::common::plaintext_util;
use cryptopals::xor;

fn main() -> io::Result<()> {
    let buffer = base64::decode(&io::stdin().lines().collect::<Result<String, _>>()?)?;

    // Guess keysize
    let mut averaged_normalised_hamming_distances = vec![f64::MAX; 64];
    for key_size in 2..=40 {
        let mut normalized_hamming_distances = vec![];
        let lhs_chunks = buffer.chunks_exact(key_size);
        let rhs_chunks = buffer[key_size..].chunks_exact(key_size);

        for (lhs, rhs) in lhs_chunks.zip(rhs_chunks) {
            normalized_hamming_distances
                .push(bits::hamming_distance(lhs, rhs) as f64 / key_size as f64);
        }
        averaged_normalised_hamming_distances[key_size] =
            normalized_hamming_distances.iter().sum::<f64>()
                / normalized_hamming_distances.len() as f64;
    }

    let key_size = averaged_normalised_hamming_distances
        .iter()
        .enumerate()
        .min_by(|x, y| x.1.partial_cmp(y.1).unwrap())
        .unwrap()
        .0;

    // Create transposed buffers
    let mut single_key_xor_buffers = vec![vec![]; key_size];
    for chunk in buffer.chunks(key_size) {
        for (idx, b) in chunk.iter().enumerate() {
            single_key_xor_buffers[idx].push(*b);
        }
    }

    // Pick best-scoring transposed plaintexts
    let mut transposed_plaintext_buffers = vec![];
    for ciphertext in single_key_xor_buffers {
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

        transposed_plaintext_buffers.push(plaintext_bytes);
    }

    // Reconstruct
    let mut plaintext_buffer = vec![0; buffer.len()];
    for (offset, transposed_plaintext) in transposed_plaintext_buffers.iter().enumerate() {
        for (idx, byte) in transposed_plaintext.iter().enumerate() {
            plaintext_buffer[offset + key_size * idx] = *byte;
        }
    }

    println!(
        "{}",
        String::from_utf8(plaintext_buffer).expect("Error building string from plaintext")
    );

    Ok(())
}
