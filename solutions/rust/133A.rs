use std::{io, process};

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let instructions: [char; 3] = ['H', 'Q', '9'];
    let input = s.chars().collect::<Vec<char>>();

    for c in instructions {
        if input.contains(&c) {
            println!("YES");
            process::exit(0);
        }
    }
    println!("NO");
}
