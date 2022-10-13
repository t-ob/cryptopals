use std::env;
use std::io;

use cryptopals::hex;
use cryptopals::xor;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let lhs = match hex::decode(&args[1]) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(io::Error::new(io::ErrorKind::Other, "DecodeError"));
        }
    };
    let rhs = match hex::decode(&args[2]) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(io::Error::new(io::ErrorKind::Other, "DecodeError"));
        }
    };

    let result = xor::xor_buffers(&lhs, &rhs);

    println!("{}", hex::encode(&result));

    Ok(())
}