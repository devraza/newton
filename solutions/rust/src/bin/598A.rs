use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let t: i64 = s.trim().parse().unwrap();

    let mut total = 0;

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let n: i64 = s.trim().parse().unwrap();

        total += n * (n + 1) / 2;

        let mut exponent = 0;
        while 2_i64.pow(exponent) <= n {
            exponent += 1;
            total -= 2_i64.pow(exponent);
        }

        println!("{}", total);
        total = 0;
    }
}
