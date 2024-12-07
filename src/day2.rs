use std::io;

fn is_increasing(x: i32, y: i32) -> bool {
    (y - x) >= 1 && (y - x) <= 3
}

fn is_decreasing(x: i32, y: i32) -> bool {
    (y - x) >= -3 && (y - x) <= -1
}

fn check_sequence(report: &Vec<i32>) -> bool {
    let diffs: Vec<_> = report.windows(2).map(|v| v[1] - v[0]).collect();
    diffs.iter().all(|x| *x >= 1 && *x <= 3) || diffs.iter().all(|x| *x <= -1 && *x >= -3)
}

fn count_strikes(report: &Vec<i32>, cmp: fn(x: i32, y:i32) -> bool) -> i32 {
    let mut strikes = 0;
    let mut prev = report[0];
    for i in 1..report.len() {
        let curr = report[i];
        if !cmp(prev, curr) {
            strikes += 1;
            if strikes > 1 {
                break;
            }
        }
        else {
            prev = curr;
        }
    }
    strikes
}

pub fn run() {
    let input = io::stdin().lines();

    let mut count_safe = 0;
    let mut count_safe_2= 0;
    
    for res in input {
        let line = res.expect("Could not read line");
        let report: Vec<i32> = line.split(" ").map(|x| x.parse().expect("Not an int.")).collect();
        if check_sequence(&report) {
            count_safe += 1;
        }
        let strikes = 0;
        for i in 0..report.len() {
            if check_sequence(&[&report[..i], &report[i+1..]].concat()) {
                count_safe_2 += 1;
                break;
            }
        }
    }
    println!("Part 1: {count_safe}");
    println!("Part 2: {count_safe_2}");

}