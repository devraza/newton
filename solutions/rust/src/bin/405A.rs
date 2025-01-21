use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut initial = s.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    initial.sort();

    let output = initial.iter().map(|d| d.to_string()).collect::<Vec<String>>();

    println!("{}", output.join(" "));
}
