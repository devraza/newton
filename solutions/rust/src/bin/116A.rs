use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut maximum = 0;
    let mut people = 0;

    for _ in 0..s.trim().parse::<u64>().unwrap() {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let s: Vec<i64> = s.split_whitespace().map(|d| d.parse::<i64>().unwrap()).collect();

        people += s[1]-s[0];

        if people > maximum {
            maximum = people;
        }
    }

    println!("{}", maximum);
}
