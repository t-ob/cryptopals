use std::io;

use cryptopals::base64;

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    for line in io::stdin().lines() {
        buffer += &line?
    }

    match base64::decode(&buffer) {
        Ok(decoded) => {
            let mut decoded_string = String::new();
            for b in decoded.iter() {
                if b.is_ascii_alphanumeric() || b.is_ascii_punctuation() || b.is_ascii_whitespace() {
                    decoded_string.push(char::from_u32(*b as u32).unwrap());
                } else {
                    decoded_string.push_str(&format!("\\x{:X}", b))
                }
            }

            println!("{:}", decoded_string)
        }
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(io::Error::new(io::ErrorKind::Other, "DecodeError"));
        }
    }

    Ok(())
}
