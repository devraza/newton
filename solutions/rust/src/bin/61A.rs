use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let a: Vec<u8> = s.trim().chars().map(|d| d as u8).collect();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let b: Vec<u8> = s.trim().chars().map(|d| d as u8).collect();

    let mut s = String::new();

    for i in 0..a.len() {
        if a[i] != b[i] {
            s.push_str("1");
        } else {
            s.push_str("0");
        }
    }

    println!("{}", s);
}
