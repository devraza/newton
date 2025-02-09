use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<usize>().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let a = s
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut maximum = 1;
    let mut count = 1;

    for i in 1..n {
        if a[i] >= a[i - 1] {
            count += 1;
            if count > maximum {
                maximum = count;
            }
        } else {
            count = 1;
        }
    }

    println!("{}", maximum);
}
