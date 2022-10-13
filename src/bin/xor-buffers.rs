use std::env;
use std::io;

use cryptopals::hex;
use cryptopals::xor;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let lhs = hex::decode(&args[1])?;
    let rhs = hex::decode(&args[2])?;

    let result = xor::xor_buffers(&lhs, &rhs);

    println!("{}", hex::encode(&result));

    Ok(())
}