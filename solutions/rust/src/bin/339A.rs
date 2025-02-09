use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut nums = s
        .trim()
        .split('+')
        .map(String::from)
        .collect::<Vec<String>>();

    nums.sort();

    println!("{}", nums.join("+"));
}
