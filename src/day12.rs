use std::io;
use std::fmt;
use std::collections::HashMap;

type Grid<T> = Vec<Vec<T>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West
}

fn print_grid<T: fmt::Display>(grid: &Grid<T>) {
    for row in grid {
        for elt in row {
            print!("{elt:3}");
        }
        println!();
    }
}

fn get_neighbors(row: usize, col: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if row > 0 {
        neighbors.push((row - 1, col));
    }
    if row < height - 1 {
        neighbors.push((row + 1, col));
    }
    if col > 0 {
        neighbors.push((row, col - 1));
    }
    if col < width - 1 {
        neighbors.push((row, col + 1));
    }
    neighbors
}

fn flood_fill(garden: &Grid<char>, regions: &mut Grid<u32>, row: usize, col: usize, reg_id: u32) -> u32 {
    let height = garden.len();
    let width = garden[0].len();
    let mut area = 0;
    let mut neighbors: Vec<(usize, usize)> = vec![(row, col)];
    let plant_type = garden[row][col];
    while let Some((row, col)) = neighbors.pop() {
        regions[row][col] = reg_id;
        for (next_row, next_col) in get_neighbors(row, col, height, width) {
            if garden[next_row][next_col] == plant_type && regions[next_row][next_col] == 0 {
                neighbors.push((next_row, next_col));
            }
        }
    }
    area
}

fn get_areas(regions: &Grid<u32>) -> HashMap<u32, u32> {
    let height = regions.len();
    let width = regions[0].len();
    let mut areas: HashMap<u32, u32> = HashMap::new();
    for row in 0..height {
        for col in 0..width {
            match areas.get(&regions[row][col]) {
                Some(val) => {
                    areas.insert(regions[row][col], val + 1);
                },
                None => {
                    areas.insert(regions[row][col], 1);
                },
            }
        }
    }
    areas
}

fn get_regions(garden: &Grid<char>) -> Grid<u32> {
    let height = garden.len();
    let width = garden[0].len();

    let mut regions: Grid<u32> = vec![vec![0; width]; height];
    let mut reg_id = 1;
    for row in 0..garden.len() {
        for col in 0..garden[0].len() {
            if regions[row][col] == 0 {
                let area = flood_fill(&garden, &mut regions, row, col, reg_id);
                reg_id += 1;
            }
        }
    }
    regions
}

fn get_perimeters(regions: &Grid<u32>) -> HashMap<u32, u32> {
    let height = regions.len();
    let width = regions[0].len();
    let mut perimeters: HashMap<u32, u32> = HashMap::new();
    for row in 0..height {
        for col in 0..width {
            let neighbors = get_neighbors(row, col, height, width);
            let mut same = 0;
            for (x, y) in neighbors {
                if regions[x][y] == regions[row][col] {
                    same += 1;
                }
            }
            let per = 4 - same;
            match perimeters.get(&regions[row][col]) {
                Some(val) => {
                    perimeters.insert(regions[row][col], val + per);
                },
                None => {
                    perimeters.insert(regions[row][col], per);
                },
            }
        }
    }
    perimeters
}

fn compute_price(areas: HashMap<u32, u32>, perimeters: HashMap<u32, u32>) -> u32 {
    let mut total_price = 0;
    for (reg_id, area) in areas {
         let perimeter = perimeters.get(&reg_id).unwrap();
         total_price += area * perimeter;
    }
    total_price
}

#[derive(Clone, Copy, Debug)]
struct Sides {
    north: Option<u32>,
    east: Option<u32>,
    south: Option<u32>,
    west: Option<u32>,
}

impl Sides {
    fn new() -> Self {
        Self {
            north: None,
            east: None,
            south: None,
            west: None,
        }
    }
}

#[derive(Debug)]
struct Directions {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl Directions {
    fn new() -> Self {
        Self {
            north: true,
            east: true,
            south: true,
            west: true,
        }
    }
}

fn compute_complement(regions: &Grid<u32>, row: usize, col: usize, neighbors: Vec<(usize, usize)>) -> Directions {
    let mut directions = Directions::new();
    for (r, c) in neighbors.into_iter().filter(|(r, c)| regions[*r][*c] == regions[row][col]) {
        if row < r {
            directions.south = false; 
        } 
        else if r < row {
            directions.north = false;
        }
        else if col < c {
            directions.east = false;
        }
        else if c < col {
            directions.west = false;
        }
    }
    directions
}

fn get_edge_sides(regions: &Grid<u32>, point: (usize, usize)) -> Directions {
    let height = regions.len();
    let width = regions[0].len();
    let (row, col) = point;
    let neighbors = get_neighbors(row, col, height, width);
    let complement = compute_complement(regions, row, col, neighbors);
    complement
}

fn get_perpendicular_neighbors(height: usize, width: usize, row: usize, col: usize, direction: Direction) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    match direction {
        Direction::East |
        Direction::West => {
            if row > 0 {
                neighbors.push((row - 1, col));
            }
            if row < height - 1 {
                neighbors.push((row + 1, col));
            }
        },
        Direction::North |
        Direction::South => {
            if col > 0 {
                neighbors.push((row, col - 1));
            }
            if col < width - 1 {
                neighbors.push((row, col + 1));
            }
        }
    }
    neighbors
}

fn count_sides(regions: &Grid<u32>, height: usize, width: usize, edge_points: &Vec<(usize, usize)>) -> u32 {
    let mut side_id = 0;

    let mut prev = edge_points[0];
    let mut prev_complement = compute_complement(regions, prev.0, prev.1, get_neighbors(prev.0, prev.1, height, width));
    for i in 1..edge_points.len() {
        let curr = edge_points[i];
        let curr_complement = compute_complement(regions, curr.0, curr.1, get_neighbors(curr.0, curr.1, height, width));
        if prev.1 + 1 != curr.1 || prev.0 != curr.0 || !prev_complement.north || !curr_complement.north {
            side_id += 1;
        }
        prev = curr;
        prev_complement = curr_complement;
    }
    side_id
}

fn get_edge_points(regions: &Grid<u32>) -> HashMap<u32, Vec<(usize, usize)>> {
    let height = regions.len();
    let width = regions[0].len();
    let mut edge_points: HashMap<u32, Vec<(usize, usize)>> = HashMap::new();
    for row in 0..height {
        for col in 0..width {
            let neighbors: Vec<(usize, usize)> = get_neighbors(row, col, height, width).into_iter().filter(|(r, c)| regions[*r][*c] == regions[row][col]).collect();
            if neighbors.len() != 4 {
                let reg_id = regions[row][col];
                match edge_points.get_mut(&reg_id) {
                    Some(points) => {
                        points.push((row, col));
                    },
                    None => {
                        edge_points.insert(reg_id, vec![(row, col)]);
                    }
                };
            }
        }
    }
    edge_points
}

fn compute_is_edge(height: usize, width: usize, edge_points: &HashMap<u32, Vec<(usize, usize)>>) -> Grid<u8> {
    let mut is_edge: Grid<u8> = vec![vec![0; width]; height];
    for (_, points) in edge_points {
        for (row, col) in points {
            is_edge[*row][*col] = 1;
        } 
    }
    is_edge
}

fn get_sides(regions: &Grid<u32>) -> HashMap<u32, u32> {
    let mut sides: HashMap<u32, u32> = HashMap::new();
    let height = regions.len();
    let width = regions[0].len();

    let edge_points = get_edge_points(regions);
    let is_edge: Grid<u8> = compute_is_edge(height, width, &edge_points); 

    for (reg_id, edge_points) in edge_points {
        let num_sides = count_sides(&regions, height, width, &edge_points);
        sides.insert(reg_id, num_sides);
    }

    sides
}

pub fn run() {
    let lines = io::stdin().lines().map(|x| x.unwrap());
    let mut garden: Grid<char> = Vec::new();
    for line in lines {
        garden.push(line.chars().collect());
    }
    let regions = get_regions(&garden);
    let areas = get_areas(&regions);
    let perimeters = get_perimeters(&regions);
    let price = compute_price(areas, perimeters);
    println!("Part 1: {price}");
    let sides = get_sides(&regions);
    println!("{sides:?}");

}