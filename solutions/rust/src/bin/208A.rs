use std::io;
use regex::Regex;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let re = Regex::new(r"WUB*").unwrap();
    let result = re.replace_all(s.trim(), " ");

    let new = result.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>().join(" ");

    println!("{}", new);
}
