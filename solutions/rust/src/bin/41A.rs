use std::io;

fn main() {
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).unwrap();

    let mut s2 = String::new();
    io::stdin().read_line(&mut s2).unwrap();

    let rev: String = s1.trim().chars().rev().collect();

    if s2.trim() == rev {
        println!("YES");
    } else {
        println!("NO");
    }
}
