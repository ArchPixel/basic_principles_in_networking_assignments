use sha3::{ Digest, Sha3_256};
use hex;
fn main() {
    // create a SHA3-256 object
    let mut hasher = Sha3_256::new();

    // get the message from the terminal
    let mut message = String::new();
    println!("Please enter the message you want to use as sha3-256 input:");
    println!("-----------------------------------------------------------");
    std::io::stdin().read_line(&mut message).expect("Failed to read line!");

    // input message
    hasher.update(&message);

    // read hash digest and transfer it into hexadecimal string
    let result = hasher.finalize();
    let output = hex::encode(result);

    // print hash digest
    println!("                                ");
    println!("The sha3-256 digest:");
    println!("--------------------");
    println!("{}",output);
}