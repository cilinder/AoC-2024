use std::io;
use regex::Regex;

fn check_test(test: u64, nums: &Vec<u64>) -> bool {
    fn helper(i: usize, test: &u64, nums: &Vec<u64>, total: u64) -> bool {
        if i >= nums.len() {
            return total == *test;
        }
        if total >= *test {
            return false;
        }
        let add_ith = helper(i+1, test, nums, total + nums[i]);
        let multiply_ith = helper(i+1, test, nums, total * nums[i]);
        return add_ith || multiply_ith
        
    }
    return helper(0, &test, &nums, 0)

}

pub fn run() {
    let lines = io::stdin().lines().map(|x| x.unwrap());
    let re = Regex::new(r"(\d+)").unwrap();

    let mut sum = 0;
    for line in lines {
        let mut captures = re.captures_iter(&line);
        let test: u64 = captures.next().unwrap()[0].parse().unwrap();
        let nums: Vec<u64> = captures.map(|x| x[0].trim().parse().unwrap()).collect();
        if check_test(test, &nums) {
            sum += test;
        }
        println!("{test} {nums:?} {}", check_test(test, &nums));
    }
    println!("Part 1: {sum}");

}