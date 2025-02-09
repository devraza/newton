use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let t: u64 = s.trim().parse().unwrap();

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let a: Vec<u64> = s
            .split_whitespace()
            .map(|d| d.parse::<u64>().unwrap())
            .collect();

        if a[0] % 2 == a[1] % 2 {
            if a[1].pow(2) > a[0] {
                println!("NO");
            } else {
                println!("YES");
            }
        } else {
            println!("NO");
        }
    }
}
