use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let a: Vec<u64> = s
        .split_whitespace()
        .map(|d| d.parse::<u64>().unwrap())
        .collect();

    let mut evenness = vec![];
    for i in a {
        if i % 2 == 0 {
            evenness.push(0);
        } else {
            evenness.push(1);
        }
    }

    let odds = evenness.iter().filter(|&n| *n == 1).count();

    if odds == 1 {
        println!("{}", evenness.iter().position(|&n| n == 1).unwrap() + 1);
    } else {
        println!("{}", evenness.iter().position(|&n| n == 0).unwrap() + 1);
    }
}
