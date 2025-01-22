use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let balance = s.trim().parse::<String>().unwrap();

    if balance.len() == 2 {
        println!("{}", balance);
        std::process::exit(0);
    }

    let removed: String = balance
        .chars()
        .take(balance.len()-2)
        .chain(balance.chars().skip(balance.len()-1))
        .collect();

    let options_str: [&str; 3] = [&balance[0..balance.len()-1], &removed, &balance[0..balance.len()]];
    let options = options_str.map(|i| i.parse::<i64>().unwrap());

    println!("{}", options.iter().max().unwrap());
}
