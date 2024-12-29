
use std::io;
use std::collections::HashMap;

#[derive(Debug)]
enum StepResult {
    Stone(u128),
    StonePair(u128, u128)
}

fn ndigits(mut num: u128) -> u32 {
    let mut counter = 0;
    while num > 0 {
        num /= 10;
        counter += 1;
    }
    counter
}

fn split_number(num: u128, n_digits: u32) -> StepResult {
    let ten:u128 = 10;
    StepResult::StonePair(num / ten.pow(n_digits / 2), (num % ten.pow(n_digits / 2)))
}

fn step(num: u128) -> StepResult {
    if num == 0 {
        return StepResult::Stone(1);
    }
    else {
        let d = ndigits(num);
        if d % 2 == 0 {
            return split_number(num, d);
        }
        else {
            return StepResult::Stone(num * 2024);
        }
    }
}

fn run_n_steps(numbers: Vec<u128>, n: u32) -> Vec<u128> {
    // println!("{numbers:?}");
    if n <= 0 {
        return numbers;
    }
    let mut next: Vec<u128> = Vec::new();
    for num in numbers {
        match step(num) {
            StepResult::Stone(x) => next.push(x),
            StepResult::StonePair(x1, x2) => {
                next.push(x1);
                next.push(x2);
            }
        }
    }
    return run_n_steps(next, n - 1);
}

fn steps(memo: &mut HashMap<(u128, u32), usize>, stone: u128, num_steps: u32) -> usize {
    if let Some(val) = memo.get(&(stone, num_steps)) {
        return *val;
    }
    if num_steps == 0 {
        return 1;
    }
    if num_steps <= 10 {
        let n_steps = run_n_steps(vec![stone], num_steps).len();
        memo.insert((stone, num_steps), n_steps);
    } 
    match step(stone) {
        StepResult::Stone(stone) => {
            let num_stones = steps(memo, stone, num_steps - 1);
            memo.insert((stone, num_steps - 1), num_stones);
            return num_stones;
        },
        StepResult::StonePair(stone1, stone2) => {
            let num_stones1 = steps(memo, stone1, num_steps - 1);
            memo.insert((stone1, num_steps - 1), num_stones1);
            let num_stones2 = steps(memo, stone2, num_steps - 1);
            memo.insert((stone2, num_steps - 1), num_stones2);
            return num_stones1 + num_stones2;
        },
    }
}

pub fn run() {
    let mut lines = io::stdin().lines().map(|x| x.unwrap());
    let input = lines.next().unwrap(); 
    let numbers: Vec<u128> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let part1 = run_n_steps(numbers.clone(), 25).len();
    let mut memo: HashMap<(u128, u32), usize> = HashMap::new();
    let mut part2 = 0;
    for number in numbers {
        part2 += steps(&mut memo, number, 75);
    }

    println!("Part 1: {part1}");
    println!("Part 2: {}", part2);
    // let part2 = run_n_steps(vec![1], 75).len();
}
