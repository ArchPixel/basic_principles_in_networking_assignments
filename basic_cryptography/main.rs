use std::io;
fn rot13(input: &str) -> String{
    input.chars().map(|cha| {
        match cha {
            'A'..='M' | 'a'..='m' =>((cha as u8) + 13) as char,
            'N'..='Z' | 'n'..='z' =>((cha as u8) - 13) as char,
            _ => cha 
        }
    }).collect()
}
fn main() {
    let mut message = String::new();
    println!("Please enter the message you want to encrypt/decrypt:");
    println!("-----------------------------------------------------");
    io::stdin().read_line(&mut message).expect("Failed to read line!");

    println!("                                ");
    println!("The encrypted/decrypted message:");
    println!("--------------------------------");
    println!("{}",rot13(&message));
}