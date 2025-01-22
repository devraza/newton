use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let inp = s.trim().split_whitespace().map(|i| i.parse::<f64>().unwrap()).collect::<Vec<f64>>();
    
    let n = inp[0];
    let m = inp[1];
    let a = inp[2];
    let b = inp[3];

    let mut price: f64;

    if b > a {
        price = b * (n/m).floor() + a * (n%m); 
        if m > n && n*a > b {
            price = b;
        } else if n*a < b && n*a < price {
            price = a;
        } else if n*a < price {
            price = n*a;
        }
    } else {
        price = b * (n/m).ceil(); 
    }

    println!("{}", price);
}
