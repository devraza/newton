use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let a = s.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let mut l = a[0];
    let mut b = a[1];

    let mut y = 0;
    while l <= b {
        y += 1;
        b *= 2;
        l *= 3;
    }

    println!("{}", y);
}
