use std::io;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Floor,
    Obstacle
}

// impl Eq for Tile

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Tile::Obstacle,
            _ => Tile::Floor
        }
    }

    fn to_char(self: &Self) -> char {
        match self {
            Tile::Floor => '.',
            Tile::Obstacle => '#'
        }
    }
}

type Grid = Vec<Vec<Tile>>;

fn print_grid(grid: &Grid) {
    for row in grid {
        for elt in row {
            match elt {
                Tile::Floor => print!("."),
                Tile::Obstacle => print!("#"),
            }
        }
        println!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Self::North),
            '>' => Some(Self::East),
            'v' => Some(Self::South),
            '<' => Some(Self::West),
            _ => None
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    x: usize,
    y: usize,
    dir: Direction
}

fn print_path(grid: &Grid, path: &Vec<State>) {
    let mut tiles: Vec<Vec<char>> = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        tiles.push(Vec::new());
        for elt in row {
            tiles[i].push(elt.to_char());
        }
    }
    for state in path {
        tiles[state.y][state.x] = 'X';
    }
    for row in tiles {
        for elt in row {
            print!("{elt}");
        }
        println!();
    }
}

fn step(grid: &Grid, state: State) -> Option<State> {
    let m = grid.len();
    let n = grid[0].len();
    match state.dir {
        Direction::North => {
            if state.y == 0 {
                return None;
            }
            else if grid[state.y-1][state.x] == Tile::Obstacle {
                return Some(State {
                    dir: state.dir.right(),
                    ..state
                });
            }
            else {
                return Some(State {
                    y: state.y - 1,
                    ..state
                });
            }
        },
        Direction::East => {
            if state.x >= n - 1 {
                return None;
            }
            else if grid[state.y][state.x+1] == Tile::Obstacle {
                return Some(State {
                    dir: state.dir.right(),
                    ..state
                });
            }
            else {
                return Some(State {
                    x: state.x + 1,
                    ..state
                });
            }
        }
        Direction::South => {
            if state.y >= m - 1 {
                return None;
            }
            else if grid[state.y+1][state.x] == Tile::Obstacle {
                return Some(State {
                    dir: state.dir.right(),
                    ..state
                });
            }
            else {
                return Some(State {
                    y: state.y + 1,
                    ..state
                });
            }
        }
        Direction::West => {
            if state.x == 0 {
                return None;
            }
            else if grid[state.y][state.x-1] == Tile::Obstacle {
                return Some(State {
                    dir: state.dir.right(),
                    ..state
                });
            }
            else {
                return Some(State {
                    x: state.x - 1,
                    ..state
                });
            }
        }
    }
}

fn patrol(grid: &Grid, state: State) -> Vec<State> {
    let path: Vec<State> = Vec::new();
    fn helper(grid: &Grid, state: State, path: Vec<State>) -> Vec<State> {
        let mb_new_state = step(&grid, state);
        match mb_new_state {
            None => return path,
            Some(new_state) => {
                let mut new_path = path.to_vec();
                let ns = new_state.clone();
                new_path.push(new_state);
                return helper(grid, ns, new_path);
            }
        }
    }
    helper(grid, state, path)
}

fn check_if_loop(grid: &Grid, state: &State) -> bool {
    let mut visited_tiles: HashSet<State> = HashSet::new();
    let mut current_state = state.clone();
    while !visited_tiles.contains(&current_state) {
        visited_tiles.insert(current_state.clone());
        let mb_current_state = step(&grid, current_state);
        match mb_current_state {
            None => return false,
            Some(cs) => {
                current_state = cs;
            }
        }
    }
    return true;
}

fn find_possible_obstacle_locations(grid: &mut Grid, start_state: State) -> u32 {
    let patrol_path = patrol(&grid, start_state.clone());
    let m = grid.len();
    let n = grid[0].len();
    let mut counted: Vec<u32> = vec![0; m * n];
    for i in 1..patrol_path.len() {
        let State { x: x, y: y, dir: dir } = &patrol_path[i];
        grid[*y][*x] = Tile::Obstacle;
        if check_if_loop(&grid, &start_state) {
            counted[*x + n * *y] = 1;
        }
        grid[*y][*x] = Tile::Floor;
    }
    counted.iter().sum()
}

fn count_distinct(grid: &Grid, path: Vec<State>) -> u32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut visited: Vec<u32> = vec![0; m * n];
    for state in path {
        visited[state.x + state.y * n] = 1;
    }
    visited.iter().sum()
}

pub fn run() {
    let lines = io::stdin().lines().map(|x| x.unwrap());
    let mut grid: Grid = Grid::new();
    // let mut start: (usize, usize);
    let mut state: State = State {x: 0, y: 0, dir: Direction::North};
    // let mut direction
    for (i, line) in lines.enumerate() {
        grid.push(line.chars().map(|c| Tile::from_char(c)).collect());
        match line.find(|c: char| c == '^' || c == '<' || c == 'v' || c == '>') {
            Some(j) => {
                let c= line.chars().nth(j).unwrap();
                state = State {
                    x: j,
                    y: i,
                    dir: Direction::from_char(c).unwrap()
                };
            },
            None => {}
        };
        
    }
    println!("{state:?}");
    print_grid(&grid);
    let patrol_path = patrol(&grid, state.clone());
    println!();
    print_path(&grid, &patrol_path);
    let distinct = count_distinct(&grid, patrol_path);
    println!("Part 1: {distinct}");

    let possible_loops = find_possible_obstacle_locations(&mut grid, state);
    println!("Part 2: {possible_loops}")

}