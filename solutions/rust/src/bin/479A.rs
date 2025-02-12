use std::io;

fn main() {
    let mut nums: Vec<i64> = vec![];

    for _ in 0..3 {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        nums.push(s.trim().parse::<i64>().unwrap());
    }

    let results: Vec<i64> = vec![
        nums[0]+nums[1]*nums[2],
        nums[0]*(nums[1]+nums[2]),
        nums[0]*nums[1]*nums[2],
        (nums[0]+nums[1])*nums[2],
        nums[0]+nums[1]+nums[2],
    ];


    println!("{}", results.iter().max().unwrap());
}
