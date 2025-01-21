use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let input = s.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let n = input[0];
    let k = input[1];

    if k > ((n as f64/ 2.).ceil() as u64) {
        if n % 2 == 0 {
            println!("{}", (2*(k-(n/2))))
        } else {
            println!("{}", (2*(k-((n+1)/2))))
        }
    } else {
        println!("{}", (2*k-1))
    }
}
