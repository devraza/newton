use std::io;

fn share_char(a: &str, b: &str) -> bool {
    let set_a: Vec<char> = a.chars().collect();
    let set_b: Vec<char> = b.chars().collect();

    let mut check = true;
    for i in set_a.iter() {
        let mut subcheck = false;
        for j in set_b.iter() {
            if i == j {
                subcheck = true;
            }
        }
        check = subcheck;
    }
    check
}

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();

    let mut strings: Vec<[String; 2]> = vec![];

    for _ in 1..=t.trim().parse::<u64>().unwrap() {
        let mut nm = String::new();
        io::stdin().read_line(&mut nm).unwrap();

        let nm_vec: Vec<&str> = nm.split_whitespace().collect();

        let mut nm_array: [u64; 2] = [0, 0];
        for i in 0..=1 {
            nm_array[i] = nm_vec[i].parse::<u64>().unwrap();
        }

        let mut string_vec: [String; 2] = [String::new(), String::new()];
        for i in &mut string_vec {
            let mut string = String::new();
            io::stdin().read_line(&mut string).unwrap();
            *i = string.trim().to_string();
        }
        strings.push(string_vec);
    }

    for i in &mut strings {
        if !share_char(&i[0], &i[1]) {
            println!("-1");
        } else {
            let mut counter = 0;
            while !i[0].contains(&i[1]) {
                i[0] = i[0].repeat(2);
                counter += 1;
                if counter > 25 {
                    counter = 0;
                    break;
                }
            }
            println!("{}", counter);
        }
    }
}
