use std::io;
use std::collections::HashMap;

fn check_update(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Option<(usize, usize)> {
    for (i, page) in update.iter().enumerate() {
        match rules.get(&page) {
            Some(forbidden) => {
                for j in 0..i {
                    if forbidden.contains(&update[j]) {
                        return Some((i, j));
                    }
                }
            },
            None => continue
        }
    }
    None
}

pub fn run() {
    let mut lines = io::stdin().lines().map(|x| x.unwrap());
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    // let mut updates: Vec<Vec<i32>> = Vec::new();

    // Read rules
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let parts: Vec<&str> = line.split("|").collect();
        let x: i32 = parts[0].parse().unwrap();
        let y: i32 = parts[1].parse().unwrap();

        match rules.get_mut(&x) {
            Some(val) => val.push(y),
            None => {
                rules.insert(x, vec![y]);
            },
        }
    }
    // println!("{rules:?}");

    // Read updates
    let mut sum1 = 0;
    let mut sum2 = 0;

    while let Some(line) = lines.next() {
        let mut update: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        let mut changed = false;
        while let Some((i, j)) = check_update(&update, &rules) {
            changed = true;
            let temp = &update[i].clone();
            update[i] = update[j];
            update[j] = *temp;
        }
        if !changed {
            let npages = update.len();
            sum1 += update[npages / 2];
        }
        else {
            let npages = update.len();
            sum2 += update[npages / 2];
        }
        
    }

    println!("Part 1: {sum1}");
    println!("Part 2: {sum2}");

}