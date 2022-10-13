mod base64;

fn main() {
    let buffer = vec![0x4d, 0x61, 0x6e, 0x4d];
    let encoded = base64::encode(&buffer);
    let decoded = base64::decode(&encoded);
    let bad_decoded = base64::decode("abcde");
    println!(
        "{:?}, {:?}, {:?}, {:?}",
        buffer, encoded, decoded, bad_decoded
    );
}
