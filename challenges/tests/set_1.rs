mod set_1 {
    use crypto::common::{base64, hex, plaintext_utils, xor};
    use std::ops::Range;

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
}
