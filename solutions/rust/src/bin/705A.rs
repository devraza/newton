use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let n = s.trim().parse::<u64>().unwrap();

    let mut words = String::from("I hate");

    for i in 0..(n - 1) {
        if i % 2 == 0 {
            words += &String::from(" that I love");
        } else {
            words += &String::from(" that I hate");
        }
    }

    println!("{} it", words);
}
