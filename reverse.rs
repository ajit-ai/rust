fn main() {
    println!("Hello, OneCompiler!");

    let s = "hello";
    let reversed: String = s.chars().rev().collect();
    println!("Reversed string: {}", reversed);

    let s = "hello";
    let reversed: Vec<u8> = s.bytes().rev().collect();
    let reversed = String::from_utf8(reversed).unwrap();
    println!("Reversed bytes: {}", reversed);

}