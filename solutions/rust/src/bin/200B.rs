use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let n = s.trim().parse::<u64>().unwrap();

    let mut p = String::new();
    io::stdin().read_line(&mut p).unwrap();

    let drinks = p
        .split_whitespace()
        .map(|c| c.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let conc = drinks.iter().sum::<u64>() as f64 / n as f64;

    println!("{}", conc);
}
