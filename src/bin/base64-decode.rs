use std::io;

use cryptopals::base64;

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    for line in io::stdin().lines() {
        buffer += &line?
    }

    let mut decoded_string = String::new();
    for b in base64::decode(&buffer)?.iter() {
        if b.is_ascii_alphanumeric() || b.is_ascii_punctuation() || b.is_ascii_whitespace() {
            decoded_string.push(char::from_u32(*b as u32).unwrap());
        } else {
            decoded_string.push_str(&format!("\\x{:X}", b))
        }
    }

    println!("{:}", decoded_string);

    Ok(())
}
