use std::io;

use cryptopals::base64;

fn main() -> io::Result<()> {
    let mut buffer = vec![];

    for line in io::stdin().lines() {
        buffer.extend_from_slice(line?.as_bytes())
    }

    let encoded = base64::encode(&buffer);

    println!("{}", encoded);

    Ok(())
}
