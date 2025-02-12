use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let t: u64 = s.trim().parse().unwrap();

    for _ in 0..t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let mut count = 0;
    
        for i in s.chars() {
            if i == '1' {
                count += 1;
            }
        }

        println!("{}", count);
    }
}
