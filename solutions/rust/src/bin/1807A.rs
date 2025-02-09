use std::io;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();

    for _ in 1..=t.trim().parse::<i16>().unwrap() {
        let mut n = String::new();
        io::stdin().read_line(&mut n).unwrap();

        let array = n.split_whitespace().collect::<Vec<&str>>();

        let mut int_array: Vec<i8> = vec![];
        for i in array {
            int_array.push(i.parse::<i8>().unwrap());
        }

        if int_array[0] + int_array[1] == int_array[2] {
            println!("+");
        } else {
            println!("-");
        }
    }
}
