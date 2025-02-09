use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let a1 = s
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let max = a1[1];

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut a2 = s
        .trim()
        .split_whitespace()
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut cost = 0;
    let mut count = 0;
    a2.sort();

    for i in a2 {
        if count == max {
            println!("{}", -1 * cost);
            std::process::exit(0);
        } else if cost + i > cost {
            println!("{}", -1 * cost);
            std::process::exit(0);
        } else {
            count += 1;
            cost += i;
        }
    }

    println!("{}", -1 * cost);
}
