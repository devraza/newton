use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let a = s.trim().split_whitespace().map(|i| i.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut n = a[0];
    let mut m = a[1];

    let mut intersections = n*m;

    let mut steps = 0;

    while intersections > 0 {
        intersections -= (n+m)-1;
        n -= 1;
        m -= 1;

        steps += 1;
    }

    if (steps+1) % 2 == 0 {
        println!("Akshat");
    } else {
        println!("Malvika");
    }
}
