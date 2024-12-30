use std::io;
use std::collections::{HashMap, HashSet};
use crate::util::Loc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West
}

struct Map {
    map: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, loc: Loc) -> usize {
        return self.map[loc.y][loc.x];
    }
}

fn check_move(topographic_map: &Map, position: Loc, direction: Direction) -> bool {
    let Loc { x, y } = position;
    match direction {
        Direction::North => return y > 0 && topographic_map.get(position) + 1 == topographic_map.get(Loc { x, y: y - 1 }),
        Direction::South => return y < topographic_map.height - 1 && topographic_map.get(position) + 1 == topographic_map.get(Loc { x, y: y + 1 }),
        Direction::East => return x < topographic_map.width - 1 && topographic_map.get(position) + 1 == topographic_map.get(Loc { x: x + 1, y }),
        Direction::West => return x > 0 && topographic_map.get(position) + 1 == topographic_map.get(Loc { x: x - 1, y }),
    }
}

fn helper(topographic_map: &Map, ends: &mut HashSet<Loc>, curr: Loc) {
    if topographic_map.get(curr) == 9 {
        ends.insert(curr);
        return;
    }
    else {
        if check_move(topographic_map, curr, Direction::East) {
            helper(topographic_map, ends, Loc { x: curr.x + 1, y: curr.y });
        }
        if check_move(topographic_map, curr, Direction::West) {
            helper(topographic_map, ends, Loc { x: curr.x - 1, y: curr.y });
        }
        if check_move(topographic_map, curr, Direction::North) {
            helper(topographic_map, ends, Loc { x: curr.x, y: curr.y - 1 });
        }
        if check_move(topographic_map, curr, Direction::South) {
            helper(topographic_map, ends, Loc { x: curr.x, y: curr.y + 1 });
        }
    }
}

fn helper2(topographic_map: &Map, curr: Loc) -> usize {
    if topographic_map.get(curr) == 9 {
        return 1;
    }
    else {
        let mut total = 0;
        if check_move(topographic_map, curr, Direction::East) {
            total += helper2(topographic_map, Loc { x: curr.x + 1, y: curr.y });
        }
        if check_move(topographic_map, curr, Direction::West) {
            total += helper2(topographic_map, Loc { x: curr.x - 1, y: curr.y });
        }
        if check_move(topographic_map, curr, Direction::North) {
            total += helper2(topographic_map, Loc { x: curr.x, y: curr.y - 1 });
        }
        if check_move(topographic_map, curr, Direction::South) {
            total += helper2(topographic_map, Loc { x: curr.x, y: curr.y + 1 });
        }
        total
    }
}

fn find_ends(topographic_map: &Map, start: Loc) -> HashSet<Loc> {
    let mut ends: HashSet<Loc> = HashSet::new();
    if topographic_map.get(start) != 0 {
        return ends;
    }
    helper(topographic_map, &mut ends, start);
    ends
}

fn find_rating(topographic_map: &Map, start: Loc) -> usize {
    if topographic_map.get(start) != 0 {
        return 0;
    }
    helper2(topographic_map, start)
}

fn find_ratings(topographic_map: &Map, starts: &Vec<Loc>) -> usize {
    let mut sum = 0;
    for start in starts {
        sum += find_rating(&topographic_map, *start);
    }
    sum
}

fn find_trailheads(topographic_map: &Map, starts: &Vec<Loc>) -> usize {
    let mut sum = 0;
    for start in starts {
        let ends: HashSet<Loc> = find_ends(&topographic_map, *start);
        sum += ends.len();
    }
    sum
}

pub fn run() {
    let lines = io::stdin().lines().map(|x| x.unwrap());
    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut starts: Vec<Loc> = Vec::new();
    for (i, line) in lines.enumerate() {
        map.push(Vec::new());
        for (j, char) in line.chars().enumerate() {
            map[i].push(char.to_digit(10).unwrap().try_into().unwrap());
            if char == '0' {
                starts.push(Loc { x: j, y: i });
            }
        }
    }
    let height = map.len();
    let width = map[0].len(); 
    let topographic_map = Map { map, height, width };
    let num_trails = find_trailheads(&topographic_map, &starts);
    let ratings = find_ratings(&topographic_map, &starts);
    println!("Part 1: {num_trails}");
    println!("Part 2: {ratings}");
}