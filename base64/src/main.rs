use std::env;
use std::io::{self, Read};

use crypto::common::base64;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "--encode" | "-e" => {
                    let mut buffer = String::new();
                    io::stdin().read_to_string(&mut buffer)?;
                    let encoding = base64::encode(buffer.as_bytes());
                    print!("{}", encoding);
                }
                "--decode" | "-d" => {
                    let mut buffer = String::new();
                    io::stdin().read_to_string(&mut buffer)?;
                    let decoding = base64::decode(&buffer[..]).unwrap();
                    print!("{}", String::from_utf8(decoding).unwrap());
                }
                _ => panic!("Usage: b64 [encode|decode]"),
            }
        }
        _ => panic!("Usage: b64 [encode|decode]"),
    }
    Ok(())
}
