use common::base64;

fn main() {
    let string = String::from("I'm killing your brain like a poisonous mushroom");

    println!("{}", base64::base64_encode(string.as_bytes()));
}
