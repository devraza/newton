use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    print!("{}{}", s[0..1].to_uppercase(), &s[1..s.len()]);
}
