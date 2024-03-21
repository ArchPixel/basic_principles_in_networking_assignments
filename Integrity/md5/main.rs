use md5; use hex;
//function to remove white spaces in-between the string
fn rmws(input: &str) -> String{
    input.chars().filter(|c| !c.is_whitespace()).collect()
}
fn main() {
    let hexinput1 = "
    d131dd02c5e6eec4693d9a0698aff95c2fcab58712467eab4004583eb8fb7f8955ad340609f4b30283e488832571415a085125e8f7cdc99fd91dbdf280373c5b
    d8823e3156348f5bae6dacd436c919c6dd53e2b487da03fd02396306d248cda0e99f33420f577ee8ce54b67080a80d1ec69821bcb6a8839396f9652b6ff72a70";
    let s1 = rmws(hexinput1);
    println!("Input block in hexadecimal form:");println!("{}",s1.clone());
    println!("-----------------------------------------------------");

    let input1 = hex::decode(&s1);          //Transform the hexadecimal input into array of 8-bit unsigned integers 
    let digest1 = md5::compute(&input1.unwrap());                  //Comput the MD5 digest using array of 8-bit unsigned integers 

    println!("Output digest from MD5:");println!("{}",format!("{:?}",digest1));
    println!("-----------------------------------------------------");
    println!("                                                     ");

    let hexinput2 ="
    d131dd02c5e6eec4693d9a0698aff95c2fcab50712467eab4004583eb8fb7f8955ad340609f4b30283e4888325f1415a085125e8f7cdc99fd91dbd7280373c5b
    d8823e3156348f5bae6dacd436c919c6dd53e23487da03fd02396306d248cda0e99f33420f577ee8ce54b67080280d1ec69821bcb6a8839396f965ab6ff72a70";
    let s2 = rmws(hexinput2);
    println!("Input block in hexadecimal form:");println!("{}",s2.clone());
    println!("-----------------------------------------------------");

    let input2 = hex::decode(&s2);           //Transform the hexadecimal input into array of 8-bit unsigned integers
    let digest2 = md5::compute(&input2.unwrap());                   //Comput the MD5 digest using array of 8-bit unsigned integers 

    println!("Output digest from MD5:");println!("{}",format!("{:?}",digest2));
    println!("-----------------------------------------------------");
}