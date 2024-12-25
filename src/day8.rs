use std::io;
use std::collections::{HashMap, HashSet};

type Grid = Vec<Vec<char>>;

fn print_grid(grid: &Grid) {
    print!("  ");
    for i in 0..grid[0].len() {
        print!("{} ", i % 10);
    }
    println!();
    for (i, row) in grid.iter().enumerate() {
        print!("{} ", i % 10);
        for elt in row {
            print!("{elt} ");
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Loc {
    x: i32,
    y: i32,
}

struct Pairs<T> {
    items: Vec<T>,
    i: usize,
    j: usize,
}

impl<T> Pairs<T> {
    fn new(items: Vec<T>) -> Pairs<T> {
        Pairs { items, i: 0, j: 0 }
    }
}

impl<T: Copy> Iterator for Pairs<T> {
    type Item = (T, T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.j >= self.items.len() - 1 {
            self.i += 1;
            self.j = self.i + 1;
        }
        else {
            self.j = self.j + 1;
        }
        if self.i >= self.items.len() - 1 {
            return None;
        }
        return Some((self.items[self.i], self.items[self.j]));
    }
}

type Frequencies = HashMap<char, Vec<Loc>>;

fn test_antinode(m: i32, n: i32, an: Loc) -> Option<Loc> {
    let Loc { x, y } = an;
    if x < 0 || y < 0 || x >= n || y >= m {
        return None;
    }
    else {
        return Some(Loc { x, y });
    }
}

fn find_antinode_pair(m: i32, n: i32, p1: Loc, p2: Loc) -> (Option<Loc>, Option<Loc>) {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let antinode1 = Loc { x: p1.x + dx, y: p1.y + dy };
    let antinode2 = Loc { x: p2.x - dx, y: p2.y - dy };
    return (test_antinode(m, n, antinode1), test_antinode(m, n, antinode2));
}

fn find_antinodes(m: i32, n: i32, p1: Loc, p2: Loc, locations: &mut HashSet<Loc>) {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let mut next = p1.clone();
    while let Some(_) = test_antinode(m, n, next) {
        locations.insert(next);
        next.x += dx;
        next.y += dy;
    }
    let mut next = p2.clone();
    while let Some(_) = test_antinode(m, n, next) {
        locations.insert(next);
        next.x -= dx;
        next.y -= dy;
    }
}

fn find_all_antinode_pairs(m: i32, n: i32, frequencies: Frequencies) -> HashSet<Loc> {
    let mut unique_locations: HashSet<Loc> = HashSet::new();
    for (_frequency, locs) in frequencies {
        for (p1, p2) in Pairs::new(locs) {
            let (mb_a1, mb_a2) = find_antinode_pair(m, n, p1, p2);
            if let Some(a1) = mb_a1 {
                unique_locations.insert(a1);
            }
            if let Some(a2) = mb_a2 {
                unique_locations.insert(a2);
            }
        }
    } 
    unique_locations
}

fn find_all_antinodes(m: i32, n: i32, frequencies: Frequencies) -> HashSet<Loc> {
    let mut unique_locations: HashSet<Loc> = HashSet::new();
    for (_frequency, locs) in frequencies {
        for (p1, p2) in Pairs::new(locs) {
            find_antinodes(m, n, p1, p2, &mut unique_locations);
        }
    } 
    unique_locations
}


pub fn run() {
    let lines = io::stdin().lines().map(|x| x.unwrap());
    let mut frequencies: Frequencies = HashMap::new();
    let mut grid: Grid = Vec::new();
    for line in lines {
        grid.push(line.chars().collect());
    }
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '.' {
                match frequencies.get_mut(&grid[i][j]) {
                    Some(locs) => {
                        let x: i32 = i32::try_from(j).unwrap();
                        let y: i32 = i32::try_from(i).unwrap(); 
                        locs.push(Loc { x, y });
                    },
                    None => {
                        let x: i32 = i32::try_from(j).unwrap();
                        let y: i32 = i32::try_from(i).unwrap(); 
                        frequencies.insert(grid[i][j], vec![Loc { x, y }]);
                    }
                }
            }
        }
    }
    let m: i32 = i32::try_from(grid.len()).unwrap();
    let n: i32 = i32::try_from(grid[0].len()).unwrap();
    let unique_pair_locations = find_all_antinode_pairs(m, n, frequencies.clone());
    let unique_locations = find_all_antinodes(m, n, frequencies);
    println!("Part 1: {}", unique_pair_locations.len());
    println!("Part 2: {}", unique_locations.len());

}