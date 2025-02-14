use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let a: Vec<f64> = s.split_whitespace().map(|d| d.parse::<f64>().unwrap()).collect();

    let n = a[0];
    let x = a[1];

    let mut count = 0;
    for i in 1..=(n.trunc() as u64) {
        let pos = x / (i as f64);
        if pos.fract() == 0. && pos <= n {
            count += 1
        }
    }

    println!("{}", count);
}
