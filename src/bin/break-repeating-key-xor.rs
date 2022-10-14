use std::env;
use std::io;
use std::io::Read;

use cryptopals::base64;
use cryptopals::hex;

fn main() -> io::Result<()> {
    //    let args: Vec<String> = env::args().collect();
    //
    //    let key = &args[1];

    let buffer = base64::decode(&io::stdin().lines().collect::<Result<String, _>>()?)?;

    //    let ciphertext: Vec<u8> = key
    //        .as_bytes()
    //        .iter()
    //        .cycle()
    //        .zip(buffer.as_bytes())
    //        .map(|(b, p)| b ^ p)
    //        .collect();
    //
    //    println!("{}", hex::encode(&ciphertext));

    Ok(())
}
