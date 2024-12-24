use std::io;
use regex::Regex;

fn count_digits(n: u128) -> u32 {
    let mut ndigits = 0;
    let mut n = n.clone();
    while n > 0 {
        n = n / 10;
        ndigits += 1;
    }
    ndigits
}

fn cat(m: u128, n: u128) -> u128 {
    let d = count_digits(n);
    let base: u128 = 10;
    m * base.pow(d) + n
}

fn check_test(test: u128, nums: &Vec<u128>) -> bool {
    fn helper(i: usize, test: &u128, nums: &Vec<u128>, total: u128) -> bool {
        if i >= nums.len() {
            return total == *test;
        }
        if total > *test {
            return false;
        }
        let add_ith = helper(i+1, test, nums, total + nums[i]);
        let multiply_ith = helper(i+1, test, nums, total * nums[i]);
        return add_ith || multiply_ith
        
    }
    return helper(1, &test, &nums, nums[0])
}

fn check_test2(test: u128, nums: &Vec<u128>) -> bool {
    fn helper(i: usize, test: &u128, nums: &Vec<u128>, total: u128) -> bool {
        if i >= nums.len() {
            return total == *test;
        }
        if total > *test {
            return false;
        }
        return helper(i+1, test, nums, total + nums[i])
            || helper(i+1, test, nums, total * nums[i])
            || helper(i+1, test, nums, cat(total, nums[i]))
        
    }
    return helper(1, &test, &nums, nums[0])
}

pub fn run() {
    let lines = io::stdin().lines().map(|x| x.unwrap());
    let re = Regex::new(r"(\d+)").unwrap();

    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in lines {
        let mut captures = re.captures_iter(&line);
        let test: u128 = captures.next().unwrap()[0].parse().unwrap();
        let nums: Vec<u128> = captures.map(|x| x[0].trim().parse().unwrap()).collect();
        if check_test(test, &nums) {
            sum1 += test;
        }
        if check_test2(test, &nums) {
            sum2 += test;
        }
    }
    println!("Part 1: {sum1}");
    println!("Part 2: {sum2}");

}