use std::io;

fn gcd(a: u64, b: u64) -> u64 {
    let mut set: [u64; 2] = [a, b];
    while set[1] != 0 {
        set = [set[1], set[0]%set[1]];
    }
    set[0]
}

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();

    let mut inputs: Vec<u64> = vec![];

    for _ in 1..=t.trim().parse::<u64>().unwrap() {
        let mut case = String::new();
        io::stdin().read_line(&mut case).unwrap();
        inputs.push(case.trim().parse::<u64>().unwrap());
    }

    let mut outputs: Vec<u64> = vec![];
    for i in inputs {
        let mut maximum = 0;
        let mut y = 0;
        for j in 1..i {
            let found = gcd(i,j)+j;
            if found > maximum {
                y = j;
                maximum = found;
            }
        }
        outputs.push(y);
    }

    for i in outputs {
        println!("{}", i);
    }
}
