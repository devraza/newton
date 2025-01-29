use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let steps = s.trim().parse::<u64>().unwrap();

    let mut count = 0;
    count += steps / 5;
    if steps % 5 != 0 {
        count += 1;
    }

    println!("{}", count); 
}
