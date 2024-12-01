use std::io;

pub fn run() {
    let input = io::stdin().lines();
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for res in input {
        let line = res.expect("Could not read line");
        let parts: Vec<&str> = line.split("   ").collect();
        let first: i32 = parts[0].parse().expect("Unable to parse number");
        let second: i32 = parts[1].parse().expect("Unable to parse number");
        left.push(first);
        right.push(second);
    }

    left.sort();
    right.sort();

    let mut sum = 0;
    for i in 0..left.len() {
       sum += &left[i].abs_diff(right[i]);
    }
    println!("Part 1: {sum}");

    let mut similarity = 0;
    for num in &left {
        let mut i = 0;
        while i < right.len() && &right[i] < num {
           i += 1;
        }
        let mut count = 0;
        while i < right.len() && &right[i] == num {
            count += 1;
            i += 1;
        }
        similarity += num * count;
    }
    println!("Part 2: {similarity}");
}