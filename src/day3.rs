use std::io;
use regex::Regex;

pub fn run() {
    let input = io::stdin().lines();
    let re = Regex::new(r"(don't\(\)|do\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut total1 = 0;
    let mut total2 = 0;
    let mut active = true;

    for res in input {
        let line = res.expect("Invalid input");
        for b in re.captures_iter(&line) {
            match &b[0] {
                "do()" => active = true,
                "don't()" => active = false,
                _ => {
                    let x: i32 = b[2].parse().expect("Not a number");
                    let y: i32 = b[3].parse().expect("Not a number");
                    total1 += x * y;
                    if active {
                        total2 += x * y;
                    }
                },
            };
        }

    }
    println!("Part 1: {total1}");
    println!("Part 2: {total2}");
}