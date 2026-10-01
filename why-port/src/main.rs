use std::env;
fn main() {
    let argm: Vec<String> = env::args().collect();
    println!("{}", &argm[1]);
    let hex: u32 = argm[1].parse().unwrap();
    let hex = format!("{:X}", &hex);
    println!("In Hexa {}", &hex);
}
