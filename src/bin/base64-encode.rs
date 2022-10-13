use std::env;
use std::io;

use cryptopals::base64;
use cryptopals::hex;

enum Mode {
    Text,
    Hex,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mode = match args.get(1) {
        Some(s) if *s == "-h".to_string() => Mode::Hex,
        _ => Mode::Text,
    };

    let mut buffer = vec![];

    for line in io::stdin().lines() {
        match mode {
            Mode::Text => buffer.extend_from_slice(line?.as_bytes()),
            Mode::Hex => {
                match hex::decode(&line?) {
                    Ok(bytes) => buffer.extend(bytes),
                    Err(err) => {
                        eprintln!("{:?}", err);
                        return Err(io::Error::new(io::ErrorKind::Other, "DecodeError"));
                    }
                }

            }
        }
    }

    let encoded = base64::encode(&buffer);

    println!("{}", encoded);

    Ok(())
}
