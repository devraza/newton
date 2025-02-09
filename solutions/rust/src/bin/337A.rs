use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let a = s
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let n: usize = a[0] as usize;
    let m: usize = a[1] as usize;

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut f = s
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    f.sort();

    let mut minimum = f[f.len() - 1] - f[0];

    for i in 0_usize..(m - n) + 1 {
        let subset = &f[i..(i + n)];
        let current = subset[subset.len() - 1] - subset[0];
        if current < minimum {
            minimum = current;
        }
    }

    println!("{}", minimum);
}
