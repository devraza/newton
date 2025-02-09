use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let n = s.trim().parse::<u64>().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut a: Vec<u64> = s
        .split_whitespace()
        .map(|d| d.parse::<u64>().unwrap())
        .collect();

    a.sort();
    a.reverse();

    let mut count = 0;
    let mut sum = 0;

    if a.iter().sum::<u64>() < n {
        println!("-1");
        std::process::exit(0);
    }

    for i in a {
        if sum < n {
            sum += i;
            count += 1;
        }
    }

    println!("{}", count);
}
