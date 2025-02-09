use std::collections::HashSet;
use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut seen = HashSet::new();
    s.retain(|c| seen.insert(c));

    if s.trim().len() % 2 != 0 {
        println!("IGNORE HIM!");
    } else {
        println!("CHAT WITH HER!");
    }
}
