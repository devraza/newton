use std::io;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();

    let mut strings: Vec<String> = vec![];

    for _ in 1..=t.trim().parse::<u64>().unwrap() {
        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();

        strings.push(a.trim().to_string());
    }

    let mut outputs: Vec<String> = vec![];

    for i in strings {
        let original = i.chars().rev();
        let mut new = String::new();
        for j in original {
            match j {
                'q' => {
                    new.push_str("p");
                },
                'p' => {
                    new.push_str("q");
                },
                _ => {
                    new.push_str("w");
                }
            }
        }
        outputs.push(new);
    }

    for i in outputs {
        println!("{}", i);
    }
}
