mod set_1 {
    use crypto::aes;
    use crypto::common::{base64, hamming, hex, plaintext_utils, xor};
    use std::env;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::ops::Range;
    use std::path::PathBuf;

    #[test]
    fn challenge_1() {
        let input = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();
        let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(base64::encode(&input), output)
    }

    #[test]
    fn challenge_2() {
        let x = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
        let y = hex::decode("686974207468652062756c6c277320657965").unwrap();
        let z = hex::decode("746865206b696420646f6e277420706c6179").unwrap();

        let result = xor::xor_buffers(&x, &y);

        assert_eq!(result, z)
    }

    #[test]
    fn challenge_3() {
        let input =
            hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                .unwrap();
        let ascii: Range<u8> = 0..0x80;

        let highest_scoring_byte = ascii
            .map(|b| {
                let output = xor::xor_buffers(&input, &vec![b; input.len()]);

                (plaintext_utils::score_english_plaintext(&output), b)
            })
            .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
            .unwrap()
            .1;

        let best = String::from_utf8(xor::xor_buffers(
            &input,
            &vec![highest_scoring_byte; input.len()],
        ))
        .unwrap();

        assert_eq!(best, "Cooking MC's like a pound of bacon")
    }

    #[test]
    fn challenge_4() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/set_1/4.txt");

        let file = File::open(d).unwrap();

        let lines = io::BufReader::new(file).lines();

        let best_candidates = lines.map(|line| {
            let input = hex::decode(&(line.unwrap())[..]).unwrap();
            let ascii: Range<u8> = 0..0x80;

            let highest_scoring_byte = ascii
                .map(|b| {
                    let output = xor::xor_buffers(&input, &vec![b; input.len()]);
                    (plaintext_utils::score_english_plaintext(&output), b)
                })
                .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
                .unwrap();

            let best = xor::xor_buffers(&input, &vec![highest_scoring_byte.1; input.len()]);

            (highest_scoring_byte.0, best)
        });

        let answer = best_candidates
            .max_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap())
            .unwrap()
            .1;

        assert_eq!(
            String::from_utf8(answer).unwrap(),
            "Now that the party is jumping\n"
        )
    }

    #[test]
    fn challenge_5() {
        let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal"
            .as_bytes();
        let key: Vec<u8> = "ICE"
            .as_bytes()
            .into_iter()
            .cycle()
            .take(input.len())
            .map(|x| *x)
            .collect();

        let output = xor::xor_buffers(input, &key);

        assert_eq!(output, hex::decode("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap())
    }

    #[test]
    fn challenge_6() {
        let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_buf.push("resources/set_1/6.txt");

        let file = File::open(path_buf).unwrap();

        let lines = io::BufReader::new(file).lines();

        let mut ciphertext: Vec<u8> = Vec::new();

        lines.for_each(|line| {
            let chunk = base64::decode(&(line.unwrap())[..]).unwrap();
            ciphertext.extend(chunk);
        });

        // Guess key size:
        // For each key size in 2..40, partition ciphertext into pairs of chunks of key_size bytes.
        // Compute a (normalised) pairwise Hamming distance between each pair, and take their average.
        // Return the key size with the lowest score.
        let key_size = (2..40)
            .map(|key_size| {
                let lhs_chunks = ciphertext.chunks(key_size).step_by(2);
                let rhs_chunks = ciphertext.chunks(key_size).skip(1).step_by(2);
                let score = lhs_chunks
                    .zip(rhs_chunks)
                    .map(|(x, y)| hamming::distance(x, y) as f64 / key_size as f64)
                    .fold(0.0, |x, y| x + y as f64)
                    / (ciphertext.len() as f64 / (2.0 * key_size as f64));
                (key_size, score)
            })
            .min_by(|lhs_key_size_score, rhs_key_size_score| {
                lhs_key_size_score
                    .1
                    .partial_cmp(&rhs_key_size_score.1)
                    .unwrap()
            })
            .unwrap()
            .0;

        // Guess key:
        // Create key_size blocks from ciphertext, where the i-th block consists of the ciphertext bytes whose
        // index in ciphertext is congruent to i (mod key_size).
        // If the guessed key size is correct, each block is encrypted by a single-byte xor, which is recovered
        // by frequency analysis
        let key: Vec<u8> = (0..key_size)
            .map(|offset| {
                let single_xor_block: Vec<u8> = ciphertext
                    .iter()
                    .skip(offset)
                    .step_by(key_size)
                    .cloned()
                    .collect();
                let ascii: Range<u8> = 0..0x80;

                ascii
                    .map(|b| {
                        let output =
                            xor::xor_buffers(&single_xor_block, &vec![b; single_xor_block.len()]);
                        (plaintext_utils::score_english_plaintext(&output), b)
                    })
                    .max_by(|(lhs_score_byte, _), (rhs_score_byte, _)| {
                        lhs_score_byte.partial_cmp(rhs_score_byte).unwrap()
                    })
                    .unwrap()
                    .1
            })
            .collect();

        // Create xor key schedule:
        // Now we know the key, cycle it and take exactly ciphertext.len() bytes of it.
        // This is what we use to xor against the original ciphertext to recover the plaintext.
        let key_schedule: Vec<u8> = key.iter().cycle().take(ciphertext.len()).cloned().collect();
        let plaintext = String::from_utf8(xor::xor_buffers(&ciphertext, &key_schedule)).unwrap();

        assert_eq!(
            String::from_utf8(key).unwrap(),
            "Terminator X: Bring the noise"
        );
        assert!(plaintext.starts_with("I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that's my DJ Deshay cuttin' all them Z's"));
    }

    #[test]
    fn challenge_7() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/set_1/7.txt");

        let file = File::open(d).unwrap();

        let lines = io::BufReader::new(file).lines();

        let mut ciphertext: Vec<u8> = Vec::new();

        lines.for_each(|line| {
            let chunk = base64::decode(&(line.unwrap())[..]).unwrap();
            ciphertext.extend(chunk);
        });

        let mut key: [u8; 0x10] = [0; 0x10];
        key.copy_from_slice("YELLOW SUBMARINE".as_bytes());

        let plaintext_bytes = aes::decrypt_ecb(&key, &ciphertext).unwrap();

        let plaintext = String::from_utf8(plaintext_bytes).unwrap();

        assert!(plaintext.starts_with("I'm back and I'm ringin' the bell"));
    }
}
